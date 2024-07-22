package kimchi

/*
#cgo darwin LDFLAGS: -L./lib -lkimchi_verifier
#cgo linux LDFLAGS: -L./lib -lkimchi_verifier -ldl -lrt -lm

#include "lib/kimchi.h"
*/
import "C"
import (
	"unsafe"
)

// TODO(xqft): check proof size
const MAX_PROOF_SIZE = 16 * 1024
const MAX_PUB_INPUT_SIZE = 50 * 1024
const MAX_VERIFIER_INDEX_SIZE = 50 * 1024

func VerifyKimchiProof(proofBuffer [MAX_PROOF_SIZE]byte, proofLen uint, pubInputBuffer [MAX_PUB_INPUT_SIZE]byte, pubInputLen uint, verifierIndexBuffer [MAX_VERIFIER_INDEX_SIZE]byte, verifierIndexLen uint) bool {
	proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
	pubInputPtr := (*C.uchar)(unsafe.Pointer(&pubInputBuffer[0]))
	verifierIndexPtr := (*C.uchar)(unsafe.Pointer(&verifierIndexBuffer[0]))
	return (bool)(C.verify_kimchi_proof_ffi(proofPtr, (C.uint)(proofLen), pubInputPtr, (C.uint)(pubInputLen), verifierIndexPtr, (C.uint)(verifierIndexLen)))
}
