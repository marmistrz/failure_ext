# `failure_ext`
[![Build Status](https://travis-ci.org/marmistrz/failure_ext.svg?branch=master)](https://travis-ci.org/marmistrz/failure_ext)
[![Docs](https://docs.rs/failure_ext/badge.svg)](https://docs.rs/failure_ext/badge.svg)

Convenience extension traits for the failure error-handling crate

This crate makes it possible to call `.context(...)` on the following types
* `std::option::Option`
* `futures::Future` (if the `future_ext` feature is enabled)
