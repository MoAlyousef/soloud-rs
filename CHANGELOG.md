# Changelog

## [0.2.4] - 2020-12-27
### Changes
- Add derive Debug for Soloud types. Thanks @toyboot4e.
- Update dependencies.

## [0.2.3] - 2020-12-02
### Changes
- Use bitflags for SoloudFlag;

## [0.2.2] - 2020-11-22
### Changes
- Update dependencies.
- Add example using include_bytes.

## [0.2.1] - 2020-10-02
### Changes
- Add a helper trait FromExt, thanks @toyboot4e
- Add AsRef<Path> impl fro LoadExt::load() and FromExt::from_path trait methods.

## [0.2.0] - 2020-09-29
### Changes
- Pull changes from origin.
- Redirect submodule to point to origin.
- Patch origin.

## [0.1.9] - 2020-09-01
### Changes
- Accept CMake toolchain files for cross-compilation via SOLOUD_TOOLCHAIN env var.
- Update deps.

## [0.1.8] - 2020-09-01
### Changes
- Fix typo in SoloudErrorKind.
- Remove unwraps in internal code.

## [0.1.7] - 2020-09-01
### Changes
- Remove effects module for now.

## [0.1.6] - 2020-08-31
### Changes
- Add AudioAttenuator and AudioCollider overridable methods.


## [0.1.5] - 2020-08-30
### Changes
- Add Bus sound source.
- Complete implementation of audio sources.

## [0.1.4] - 2020-08-28
### Changes
- Make the Soloud struct an RAII type with a Drop impl.
- Add a changelog.
- Add CI to check builds on windows, macos and linux.