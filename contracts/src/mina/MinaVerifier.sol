// SPDX-License-Identifier: UNLICENSED
pragma solidity =0.8.12;

import {AlignedLayerServiceManager} from "../core/AlignedLayerServiceManager.sol";

contract MinaVerifier {
    function verify(
        address alignedVerifier,
        bytes32 proofCommitment,
        bytes32 pubInputCommitment,
        bytes32 provingSystemAuxDataCommitment,
        bytes20 proofGeneratorAddr,
        bytes32 batchMerkleRoot,
        bytes memory merkleProof,
        uint256 verificationDataBatchIndex
    ) public view returns (bool result) {
        AlignedLayerServiceManager alignedContract = AlignedLayerServiceManager(alignedVerifier);

        result = alignedContract.verifyBatchInclusion(
            proofCommitment,
            pubInputCommitment,
            provingSystemAuxDataCommitment,
            proofGeneratorAddr,
            batchMerkleRoot,
            merkleProof,
            verificationDataBatchIndex
        );
    }
}
