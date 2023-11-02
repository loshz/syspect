// Code generated by bpf2go; DO NOT EDIT.
//go:build 386 || amd64 || amd64p32 || arm || arm64 || loong64 || mips64le || mips64p32le || mipsle || ppc64le || riscv64

package bpf

import (
	"bytes"
	_ "embed"
	"fmt"
	"io"

	"github.com/cilium/ebpf"
)

type sys_enterMapKey struct {
	ProcName [16]int8
	Pid      int32
}

// loadSys_enter returns the embedded CollectionSpec for sys_enter.
func loadSys_enter() (*ebpf.CollectionSpec, error) {
	reader := bytes.NewReader(_Sys_enterBytes)
	spec, err := ebpf.LoadCollectionSpecFromReader(reader)
	if err != nil {
		return nil, fmt.Errorf("can't load sys_enter: %w", err)
	}

	return spec, err
}

// loadSys_enterObjects loads sys_enter and converts it into a struct.
//
// The following types are suitable as obj argument:
//
//	*sys_enterObjects
//	*sys_enterPrograms
//	*sys_enterMaps
//
// See ebpf.CollectionSpec.LoadAndAssign documentation for details.
func loadSys_enterObjects(obj interface{}, opts *ebpf.CollectionOptions) error {
	spec, err := loadSys_enter()
	if err != nil {
		return err
	}

	return spec.LoadAndAssign(obj, opts)
}

// sys_enterSpecs contains maps and programs before they are loaded into the kernel.
//
// It can be passed ebpf.CollectionSpec.Assign.
type sys_enterSpecs struct {
	sys_enterProgramSpecs
	sys_enterMapSpecs
}

// sys_enterSpecs contains programs before they are loaded into the kernel.
//
// It can be passed ebpf.CollectionSpec.Assign.
type sys_enterProgramSpecs struct {
	SysEnter *ebpf.ProgramSpec `ebpf:"sys_enter"`
}

// sys_enterMapSpecs contains maps before they are loaded into the kernel.
//
// It can be passed ebpf.CollectionSpec.Assign.
type sys_enterMapSpecs struct {
	SyscallCount *ebpf.MapSpec `ebpf:"syscall_count"`
}

// sys_enterObjects contains all objects after they have been loaded into the kernel.
//
// It can be passed to loadSys_enterObjects or ebpf.CollectionSpec.LoadAndAssign.
type sys_enterObjects struct {
	sys_enterPrograms
	sys_enterMaps
}

func (o *sys_enterObjects) Close() error {
	return _Sys_enterClose(
		&o.sys_enterPrograms,
		&o.sys_enterMaps,
	)
}

// sys_enterMaps contains all maps after they have been loaded into the kernel.
//
// It can be passed to loadSys_enterObjects or ebpf.CollectionSpec.LoadAndAssign.
type sys_enterMaps struct {
	SyscallCount *ebpf.Map `ebpf:"syscall_count"`
}

func (m *sys_enterMaps) Close() error {
	return _Sys_enterClose(
		m.SyscallCount,
	)
}

// sys_enterPrograms contains all programs after they have been loaded into the kernel.
//
// It can be passed to loadSys_enterObjects or ebpf.CollectionSpec.LoadAndAssign.
type sys_enterPrograms struct {
	SysEnter *ebpf.Program `ebpf:"sys_enter"`
}

func (p *sys_enterPrograms) Close() error {
	return _Sys_enterClose(
		p.SysEnter,
	)
}

func _Sys_enterClose(closers ...io.Closer) error {
	for _, closer := range closers {
		if err := closer.Close(); err != nil {
			return err
		}
	}
	return nil
}

// Do not access this directly.
//
//go:embed sys_enter_bpfel.o
var _Sys_enterBytes []byte
