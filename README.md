# Ploy Engine

Node graph architecture based business process automation engine, that is using gRPC communication protocol, written in RUST. It is meant to be used together with services that connect to the engine and execute jobs, that engine determines by evaluating Ploy Diagrams.


Engine supports Node Graph Architecture model validation and execution. To support data transformations during execution, it integrates Python code interpreter. Current system prototype includes process and JSON Schema repository in itself.
