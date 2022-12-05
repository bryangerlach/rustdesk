#!/usr/bin/env bash

dart pub global activate ffigen --version 5.0.1
flutter pub get
# call `flutter clean` if cargo build fails
# export LLVM_HOME=/Library/Developer/CommandLineTools/usr/
#cargo build --features flutter
cargo build --release --features flutter
flutter run $@