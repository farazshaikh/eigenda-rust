// Sticking to ethers, we will move to alloy wholesale once we know our needed features set and its
// availibility on alloy.
//
//use foundry_compilers::{
//    remappings::Remapping, resolver::print, utils, Project, ProjectPathsConfig,
//};
use anyhow::Result;
use tonic_build;

//  Build GRPC bindings for Eigen DA
//
//  # Arguements:
//  *   proto_file_path: Path to the EigenDA GRPC IDL file
//
//  # Examples:
//  *  build_eignDA("./nucleus/contracts").expect("Compilation failed");
fn build_eigenda(proto_file_path: &str) -> Result<()> {
    // Build GRPC binding for EigenDA
    tonic_build::compile_protos(proto_file_path)?;
    Ok(())
}

fn main() -> Result<()> {
    // Build Eigen DA RPC:
    // https://github.com/Layr-Labs/eigenda/blob/master/api/proto/disperser/disperser.proto
    build_eigenda("src/disperser.proto")?;
    Ok(())
}
