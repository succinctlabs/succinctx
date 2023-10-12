import { toHexString } from "@chainsafe/ssz";
import { AxiosInstance } from "axios";
import { altair, capella, phase0, ssz } from "@lodestar/types";
import {
  ProofType,
  SingleProof,
  createProof,
} from "@chainsafe/persistent-merkle-tree";

type BeaconId = number | string | Uint8Array | bigint;

export const ROUTES = {
  getBlock: "/eth/v2/beacon/blocks/{block_id}",
  getGenesis: "/eth/v1/beacon/genesis",
  getHeader: "/eth/v1/beacon/headers/{block_id}",
  getBeaconState: "/eth/v2/debug/beacon/states/{state_id}",
  getSyncStatus: "/eth/v1/node/syncing",
};

const config = {
  secondsPerSlot: 12,
  slotsPerEpoch: 32,
  epochsPerPeriod: 256,
  capellaForkEpoch: 194048,
  capellaForkSlot: 194048 * 32,
};

export type StepUpdate = {
  attestedBlock: capella.BeaconBlock;
  currentSyncCommittee: altair.SyncCommittee;
  finalizedBlock: capella.BeaconBlock;
  finalityBranch: Uint8Array[];
  syncAggregate: altair.SyncAggregate;
  genesisValidatorsRoot: Uint8Array;
  genesisTime: number;
  forkVersion: Uint8Array;
  executionStateRoot: Uint8Array;
  executionStateBranch: Uint8Array[];
};

export type RotateUpdate = {
  currentSyncCommittee: altair.SyncCommittee;
  nextSyncCommittee: altair.SyncCommittee;
  nextSyncCommitteeBranch: Uint8Array[];
  syncAggregate: altair.SyncAggregate;
  genesisValidatorsRoot: Uint8Array;
  genesisTime: number;
  forkVersion: Uint8Array;
};

export class ConsensusClient {
  client: AxiosInstance;
  slotsPerEpoch: number;
  slotsPerPeriod: number;
  SLOTS_PER_HISTORICAL_ROOT = 8192;

  constructor(
    axiosClient: AxiosInstance,
    slotsPerEpoch: number,
    slotsPerPeriod: number
  ) {
    this.client = axiosClient;
    this.slotsPerEpoch = slotsPerEpoch;
    this.slotsPerPeriod = slotsPerPeriod;
  }

  toStringFromBeaconId(identifier: any) {
    if (identifier instanceof Uint8Array) {
      return toHexString(identifier);
    }
    return identifier.toString();
  }

  async getHeader(
    blockIdentifier: BeaconId
  ): Promise<phase0.BeaconBlockHeader> {
    const id = this.toStringFromBeaconId(blockIdentifier);
    const response = await this.client.get(
      ROUTES.getHeader.replace("{block_id}", id)
    );
    const header = ssz.phase0.BeaconBlockHeader.fromJson(
      response.data.data.header.message
    );
    return header;
  }
  async getSSZ(blockIdentifier: BeaconId) {
    let slot: number;
    if (typeof blockIdentifier === "number") {
      slot = blockIdentifier;
    } else {
      const header = await this.getHeader(blockIdentifier);
      slot = header.slot;
    }
    if (slot < config.capellaForkSlot) {
      return ssz.bellatrix;
    } else {
      return ssz.capella;
    }
  }

  async getBlock(blockIdentifier: BeaconId): Promise<capella.BeaconBlock> {
    const ssz = await this.getSSZ(blockIdentifier);
    const id = this.toStringFromBeaconId(blockIdentifier);
    const response = await this.client.get(
      ROUTES.getBlock.replace("{block_id}", id)
    );
    const block = ssz.BeaconBlock.fromJson(
      response.data.data.message
    ) as capella.BeaconBlock;
    return block;
  }

  async getState(stateIdentifier: BeaconId): Promise<capella.BeaconState> {
    const ssz = await this.getSSZ(stateIdentifier);
    const id = this.toStringFromBeaconId(stateIdentifier);
    const response = await this.client.get(
      ROUTES.getBeaconState.replace("{state_id}", id),
      {
        responseType: "arraybuffer",
        headers: {
          Accept: "application/octet-stream",
        },
      }
    );
    const bytes = response.data as Buffer;
    const state = ssz.BeaconState.deserialize(bytes);
    return state as capella.BeaconState;
  }

  async getStepUpdate(attestedIdentifier: BeaconId): Promise<StepUpdate> {
    const ssz = await this.getSSZ(attestedIdentifier);

    const attestedBlock = await this.getBlock(attestedIdentifier);
    const signedSlot = attestedBlock.slot + 1;
    const signedBlock = await this.getBlock(signedSlot);
    const [attestedState, signedState] = await Promise.all([this.getState(attestedBlock.slot), this.getState(signedSlot)]);
    const attestedStateView = ssz.BeaconState.toView(attestedState as any);

    const finalizedBlock = await this.getBlock(
      attestedState.finalizedCheckpoint.root
    );
    const finalityBranchIndex = ssz.BeaconState.getPathInfo([
      "finalized_checkpoint",
      "root",
    ]).gindex;
    const finalityBranch = (
      createProof(attestedStateView.node, {
        type: ProofType.single,
        gindex: finalityBranchIndex,
      }) as SingleProof
    ).witnesses;

    const currentSyncCommittee = signedState.currentSyncCommittee;
    const syncAggregate = signedBlock.body.syncAggregate;
    const genesisTime = signedState.genesisTime;
    const genesisValidatorsRoot = signedState.genesisValidatorsRoot;
    const forkVersion =
      Math.floor(signedState.slot / this.slotsPerEpoch) <
      signedState.fork.epoch
        ? signedState.fork.previousVersion
        : signedState.fork.currentVersion;

    const executionStateRootAndBranch = await this.getExecutionStateRootProof(
      finalizedBlock
    );
    const executionStateRoot = executionStateRootAndBranch.root;
    const executionStateBranch = executionStateRootAndBranch.branch;

    return {
      attestedBlock,
      currentSyncCommittee,
      finalizedBlock,
      finalityBranch,
      syncAggregate,
      genesisValidatorsRoot,
      genesisTime,
      forkVersion,
      executionStateRoot,
      executionStateBranch,
    };
  }

  async getRotateUpdate(blockIdentifier: BeaconId): Promise<RotateUpdate> {
    const ssz = await this.getSSZ(blockIdentifier);

    const finalizedBlock = await this.getBlock(blockIdentifier);
    const finalizedState = await this.getState(finalizedBlock.slot);
    const finalizedStateView = ssz.BeaconState.toView(finalizedState as any);

    const currentSyncCommittee = finalizedState.currentSyncCommittee;
    const nextSyncCommitteeIndex = ssz.BeaconState.getPathInfo([
      "next_sync_committee",
    ]).gindex;
    const nextSyncCommittee = finalizedState.nextSyncCommittee;
    const nextSyncCommitteeBranch = (
      createProof(finalizedStateView.node, {
        type: ProofType.single,
        gindex: nextSyncCommitteeIndex,
      }) as SingleProof
    ).witnesses;

    const syncAggregate = finalizedBlock.body.syncAggregate;
    const genesisTime = finalizedState.genesisTime;
    const genesisValidatorsRoot = finalizedState.genesisValidatorsRoot;
    const forkVersion =
      Math.floor(finalizedState.slot / this.slotsPerEpoch) <
      finalizedState.fork.epoch
        ? finalizedState.fork.previousVersion
        : finalizedState.fork.currentVersion;

    return {
      currentSyncCommittee,
      nextSyncCommittee,
      nextSyncCommitteeBranch,
      syncAggregate,
      genesisValidatorsRoot,
      genesisTime,
      forkVersion,
    };
  }

  async getExecutionStateRootProof(block: capella.BeaconBlock) {
    const view = ssz.capella.BeaconBlockBody.toView(block.body as any);
    const proof = createProof(view.node, {
      type: ProofType.single,
      gindex: BigInt(402),
    }) as SingleProof;
    return { root: proof.leaf, branch: proof.witnesses };
  }
}
