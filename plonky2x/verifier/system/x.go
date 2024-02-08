// ReadFrom attempts to decode a VerifyingKey from reader
// VerifyingKey must be encoded through WriteTo (compressed) or WriteRawTo (uncompressed)
// serialization format:
// https://github.com/zkcrypto/bellman/blob/fa9be45588227a8c6ec34957de3f68705f07bd92/src/groth16/mod.rs#L143
// [α]1,[β]1,[β]2,[γ]2,[δ]1,[δ]2,uint32(len(Kvk)),[Kvk]1
func (vk *VerifyingKey) ReadFrom(r io.Reader) (int64, error) {
	n, err := vk.readFrom(r)
	if err != nil {
		return n, err
	}
	var m int64
	m, err = vk.CommitmentKey.ReadFrom(r)
	return m + n, err
}

// UnsafeReadFrom has the same behavior as ReadFrom, except that it will not check that decode points
// are on the curve and in the correct subgroup.
func (vk *VerifyingKey) UnsafeReadFrom(r io.Reader) (int64, error) {
	n, err := vk.readFrom(r, curve.NoSubgroupChecks())
	if err != nil {
		return n, err
	}
	var m int64
	m, err = vk.CommitmentKey.UnsafeReadFrom(r)
	return m + n, err
}

func (vk *VerifyingKey) readFrom(r io.Reader, decOptions ...func(*curve.Decoder)) (int64, error) {
	dec := curve.NewDecoder(r, decOptions...)

	// [α]1,[β]1,[β]2,[γ]2,[δ]1,[δ]2
	if err := dec.Decode(&vk.G1.Alpha); err != nil {
		return dec.BytesRead(), err
	}
	if err := dec.Decode(&vk.G1.Beta); err != nil {
		return dec.BytesRead(), err
	}
	if err := dec.Decode(&vk.G2.Beta); err != nil {
		return dec.BytesRead(), err
	}
	if err := dec.Decode(&vk.G2.Gamma); err != nil {
		return dec.BytesRead(), err
	}
	if err := dec.Decode(&vk.G1.Delta); err != nil {
		return dec.BytesRead(), err
	}
	if err := dec.Decode(&vk.G2.Delta); err != nil {
		return dec.BytesRead(), err
	}

	// uint32(len(Kvk)),[Kvk]1
	if err := dec.Decode(&vk.G1.K); err != nil {
		return dec.BytesRead(), err
	}
	var publicCommitted [][]uint64
	if err := dec.Decode(&publicCommitted); err != nil {
		return dec.BytesRead(), err
	}
	vk.PublicAndCommitmentCommitted = utils.Uint64SliceSliceToIntSliceSlice(publicCommitted)

	// recompute vk.e (e(α, β)) and  -[δ]2, -[γ]2
	if err := vk.Precompute(); err != nil {
		return dec.BytesRead(), err
	}

	return dec.BytesRead(), nil
}