// let builder = CircuitBuilder::new();
// let result = QueryBuilder::new()
//                 .eth()
//                 .consensus()
//                 .validators()
//                 .map(|v| {
//                     v.balance
//                 }).reduce(|a, b| a + b)
//                 .query(builder);

// builder.map<I, T>(|I::Item| {
//     T
// }).reduce(|T, T| {
//     T
// });

// let iterator = BeaconValidatorsIterator::new();
// builder.mapreduce<U64Variable>(|validator, builder| {
//     validator.balance
// }, |b1, T, builder| {
//     T
// });
