# Build config.
BIN_DIR ?= ./bin
BPF_OUT ?= ./pkg/bpf
BUILD_NUMBER ?= 0.1.0

.PHONY: go/build go/lint go/test bpf/btf bpf/build

go/build:
	CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build \
	    --ldflags="-X github.com/loshz/syspect/pkg/version.Version=$(BUILD_NUMBER)" \
		-o $(BIN_DIR)/syspect ./cmd/...

go/lint:
	@golangci-lint run --config .golangci.yml

go/test:
	@go test $(GO_TEST_FLAGS) ./...

bpf/btf:
	bpftool btf dump file /sys/kernel/btf/vmlinux format c > ./bpf/vmlinux.h

bpf/build: bpf/btf
	mkdir -p ${BPF_OUT}
