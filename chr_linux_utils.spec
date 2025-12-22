%global debug_package %{nil}

Name:           chr_linux_utils
Version:        0.1.1
Release:        1%{?dist}
Summary:        Rust implementation of basic Linux utilities (ls, mkdir, etc.)

License:        MIT
URL:            https://github.com/onetwoboth/chr_linux_utils
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  rust
BuildRequires:  cargo

Requires:       glibc

%description
chr_linux_utils is a Rust-based reimplementation of basic Linux utilities.
It currently provides:
- chrls     (ls-like command)
- chrmkdir (mkdir-like command)
- chrrm (rm-like command)

The goal of this project is learning systems programming and Linux tooling
with Rust, while staying close to real-world engineering practices.

%prep
%autosetup -n %{name}-%{version}

%build
cargo build --release

%install
rm -rf %{buildroot}

# Install binaries
install -D -m 0755 target/release/chrls %{buildroot}/usr/bin/chrls
install -D -m 0755 target/release/chrmkdir %{buildroot}/usr/bin/chrmkdir
install -D -m 0755 target/release/chrrm %{buildroot}/usr/bin/chrrm

%files
/usr/bin/chrls
/usr/bin/chrmkdir
/usr/bin/chrrm

%changelog
* Mon Dec 22 2025 Haoran Chen <209872581@qq.com> - 0.1.1-1
- Add chrrm 

* Mon Dec 22 2025 Haoran Chen <209872581@qq.com> - 0.1.0-1
- Initial RPM release of chr_linux_utils
