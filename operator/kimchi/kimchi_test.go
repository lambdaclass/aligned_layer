package kimchi_test

import (
	"fmt"
	"os"
	"testing"

	"github.com/yetanotherco/aligned_layer/operator/kimchi"
)

func TestEcAddKimchiProofVerifies(t *testing.T) {
	fmt.Println(os.Getwd())
	proofFile, err := os.Open("lib/kimchi_ec_add.proof")
	if err != nil {
		t.Errorf("could not open kimchi proof file")
	}

	proofBuffer := make([]byte, kimchi.MAX_PROOF_SIZE)
	proofLen, err := proofFile.Read(proofBuffer)
	if err != nil {
		t.Errorf("could not read bytes from kimchi proof file")
	}

	pubInputBuffer := make([]byte, kimchi.MAX_PUB_INPUT_SIZE)
	pubInputLen := 0

	verifierIndexFile, err := os.Open("lib/kimchi_verifier_index.bin")
	if err != nil {
		t.Errorf("could not open kimchi aggregated public input file")
	}
	verifierIndexBuffer := make([]byte, kimchi.MAX_VERIFIER_INDEX_SIZE)
	verifierIndexLen, err := verifierIndexFile.Read(verifierIndexBuffer)
	if err != nil {
		t.Errorf("could not read bytes from kimchi aggregated verifier index")
	}

	if !kimchi.VerifyKimchiProof(([kimchi.MAX_PROOF_SIZE]byte)(proofBuffer), uint(proofLen), ([kimchi.MAX_PUB_INPUT_SIZE]byte)(pubInputBuffer), uint(pubInputLen), ([kimchi.MAX_PUB_INPUT_SIZE]byte)(verifierIndexBuffer), uint(verifierIndexLen)) {
		t.Errorf("proof did not verify")
	}
}
