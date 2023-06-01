Name:           blueprint
Version:        0.1.0
Release:        1%{?dist}
Summary:        An out of the box developer environment designer.

License:        MPL-2.0
URL:            https://blueprint.edfloreshz.dev/
Source0:        https://github.com/edfloreshz/blueprint/releases/download/v0.1.0/blueprint-0.1.0.tar.xz

BuildRequires:  gettext
BuildRequires:  meson
BuildRequires:  ninja-build
BuildRequires:  gcc
Requires:       info

%description
Blueprint improves the initial experience developers have when starting to work with a new codebase, project or system. The initial setup and configuration can often be time-consuming and frustrating, leading to delays in productivity and a steep learning curve for newcomers. 

%prep
%autosetup -p1

%meson
%meson_build
%meson_install

%files
%{_bindir}/blueprint
%{_datadir}/applications/
%{_datadir}/blueprint/
%{_datadir}/doc/
%{_datadir}/glib-2.0/
%{_datadir}/icons/
%{_datadir}/man/
%{_datadir}/metainfo/

%license LICENSE

%changelog
* Mon May 29 2023 Eduardo Flores <edfloreshz@gmail.com>
- 
