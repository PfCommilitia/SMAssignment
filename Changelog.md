# Changelog

This file contains commit messages for the repository.

## v0.1.0

### Project Structure

* From this version, git commit message only contains version number.
Refer to this file for detailed changes.
* By migrating commit message to a separate file, changes in a time frame don't have to be
broken down to multiple commits, and even more information can be delivered.

### Features

* Add SM2 encryption and decryption functions

### Improvements

* Various implementations of `Into<_>` replaced with `From<_>`
* Improve implementations for `BitSequence` and `EccPoint`. Spotlight:
  * `BitSequence::slice`
  * `BitSequence::xor`
  * `Eq` and `PartialEq` for `BitSequence`
  * `EccPoint::validate_on_curve` and `EccPoint::validate_on_given_curve`
  * Conversions among bytes, `BitSequence` and `EccPoint`
* Change various functions to read `g` from parameters passed directly or via `EccPoint`, instead of reading from static `SM2_G`
