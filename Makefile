BIN_DIR ?= ./bin
BUILD_NUMBER ?= 0.1.0
GO_TEST_FLAGS ?= -v

.PHONY: go/build go/lint go/test bpf/btf bpf/build

go/build:
	@CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build \
	    --ldflags="-X github.com/loshz/syspect/pkg/version.Version=$(BUILD_NUMBER)" \
		-o $(BIN_DIR)/syspect ./cmd/...

go/lint:
	@golangci-lint run --config .golangci.yml

go/test:
	@go test $(GO_TEST_FLAGS) $(shell go list ./... | grep -v pkg/bpf)

bpf/btf:
	@bpftool btf dump file /sys/kernel/btf/vmlinux format c > ./bpf/vmlinux.h

bpf/build: bpf/btf
	@go generate ./pkg/bpf/
