import { capella, phase0, ssz } from "@lodestar/types";
import { AxiosInstance } from "axios";
type BeaconId = number | string | Uint8Array | bigint;
export declare const ROUTES: {
    getBlock: string;
    getGenesis: string;
    getHeader: string;
    getBeaconState: string;
    getSyncStatus: string;
};
export declare class ConsensusClient {
    client: AxiosInstance;
    slotsPerEpoch: number;
    slotsPerPeriod: number;
    SLOTS_PER_HISTORICAL_ROOT: number;
    constructor(axiosClient: AxiosInstance, slotsPerEpoch: number, slotsPerPeriod: number);
    toStringFromBeaconId(identifier: any): any;
    getHeader(blockIdentifier: BeaconId): Promise<phase0.BeaconBlockHeader>;
    getSSZ(blockIdentifier: BeaconId): Promise<typeof ssz.capella | typeof ssz.bellatrix>;
    getBlock(blockIdentifier: BeaconId): Promise<capella.BeaconBlock>;
}
export {};
//# sourceMappingURL=client.d.ts.map