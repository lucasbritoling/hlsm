## Overview

Kernel-Anchored Application Whitelisting Enforcement (Kaawe) is a high-performance, kernel-anchored runtime security sandbox 
designed for Linux environments. It implements a zero-trust, deterministic 
Static Application Whitelisting mechanism based on File Integrity Enforcement.

By leveraging eBPF's synchronous `BPF_LSM` hook (`bprm_check_security`), the system 
intercepts any executable binary execution (`execve`) directly at the kernel level. 
It validates the file's immutable identity (Device ID, Inode number, and Modification Time) 
against an in-memory eBPF HashMap populated by a secure Rust user-space component, 
safely blocking any unauthorized or modified code execution prior to its birth.
