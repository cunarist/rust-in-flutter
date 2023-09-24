#
# To learn more about a Podspec see http://guides.cocoapods.org/syntax/podspec.html.
# Run `pod lib lint rust_in_flutter.podspec` to validate before publishing.
#
Pod::Spec.new do |s|
  s.name             = 'rust_in_flutter'
  s.version          = '0.1.0'
  s.summary          = 'Summary'
  s.description      = 'Description'
  s.homepage         = 'http://cunarist.com'
  s.license          = { :file => '../LICENSE' }
  s.author           = { 'Your Company' => 'email@cunarist.com' }

  # This will ensure the source files in Classes/ are included in the native
  # builds of apps using this FFI plugin. Podspec does not support relative
  # paths, so Classes contains a forwarder C file that relatively imports
  # `../src/*` so that the C sources can be shared among all target platforms.
  s.source           = { :path => '.' }
  s.source_files     = 'Classes/**/*'
  s.dependency 'Flutter'
  
  s.platform = :ios, '11.0'
  s.swift_version = '5.0'

  script = <<-SCRIPT
      echo "Generate protobuf message" 
      cd $PROJECT_DIR/../../
      dart run rust_in_flutter message
      echo "Build Rust library"
      sh "$PODS_TARGET_SRCROOT/../cargokit/build_pod.sh" "$PROJECT_DIR/../../native/hub" hub
    SCRIPT
    
  # Include Rust crates in the build process.
  s.script_phase = {
    :name => 'Generate protobuf message and build Rust library',
    :script => script,
    :execution_position => :before_compile,
    :input_files => ['${BUILT_PRODUCTS_DIR}/cargokit_phony'],
    :output_files => ['${BUILT_PRODUCTS_DIR}/cargokit_phony_out', '${BUILT_PRODUCTS_DIR}/output.txt'],
  }
  s.pod_target_xcconfig = {
    # From default Flutter template.
    'DEFINES_MODULE' => 'YES',
    # Flutter framework does not contain a i386 slice. From default Flutter template.
    'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'i386',
    # Rust can't produce armv7 and it's being removed from Flutter as well.
    'EXCLUDED_ARCHS' => 'armv7',
    # We use `-force_load` instead of `-l` since Xcode strips out unused symbols from static libraries.
    'OTHER_LDFLAGS' => '-force_load ${BUILT_PRODUCTS_DIR}/libhub.a',
    'DEAD_CODE_STRIPPING' => 'YES',
    'STRIP_INSTALLED_PRODUCT[config=*][sdk=*][arch=*]' => "YES",
    'STRIP_STYLE[config=*][sdk=*][arch=*]' => "non-global",
    'DEPLOYMENT_POSTPROCESSING[config=*][sdk=*][arch=*]' => "YES",
  }
end
