import 'dart:io';

import 'package:flutter_app_packager/src/api/app_package_maker.dart';

// format of make_config for custom packager
/*
# The script or command to execute for packaging.
# The script receives the following environment variables:
#   APP_NAME              - the application name
#   APP_VERSION           - full version string (e.g. 1.0.0+1)
#   BUILD_NAME            - version without build number (e.g. 1.0.0)
#   BUILD_NUMBER          - build number (e.g. 1), only set when present
#   BUILD_MODE            - build mode (release / debug / profile)
#   FLAVOR                - build flavor, only set when present
#   CHANNEL               - release channel, only set when present
#   BUILD_OUTPUT_DIRECTORY - absolute path to the flutter build output directory
#   OUTPUT_DIRECTORY      - absolute path to the base output directory
#   OUTPUT_ARTIFACT_PATH  - absolute path where the output artifact is expected
script: ./scripts/package.sh

# The file extension for the output artifact (e.g. tar.gz, zip, pkg).
# Omit or leave empty to produce a directory artifact instead of a file.
output_extension: tar.gz
*/

class MakeCustomConfig extends MakeConfig {
  MakeCustomConfig({
    required this.script,
    this.outputExtension = '',
  });

  factory MakeCustomConfig.fromJson(Map<String, dynamic> map) {
    return MakeCustomConfig(
      script: map['script'] as String,
      outputExtension: (map['output_extension'] as String?) ?? '',
    );
  }

  final String script;
  final String outputExtension;
}

class MakeCustomConfigLoader extends DefaultMakeConfigLoader {
  @override
  MakeConfig load(
    Map<String, dynamic>? arguments,
    Directory outputDirectory, {
    required Directory buildOutputDirectory,
    required List<File> buildOutputFiles,
  }) {
    final map = loadMakeConfigYaml(
      '$platform/packaging/$packageFormat/make_config.yaml',
    );
    final customConfig = MakeCustomConfig.fromJson(map);

    // Override packageFormat so the output artifact path uses the right extension.
    packageFormat = customConfig.outputExtension;

    final baseMakeConfig = super.load(
      arguments,
      outputDirectory,
      buildOutputDirectory: buildOutputDirectory,
      buildOutputFiles: buildOutputFiles,
    );
    return customConfig.copyWith(baseMakeConfig);
  }
}
