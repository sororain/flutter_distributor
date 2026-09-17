import 'dart:io';

import 'package:flutter_app_packager/src/api/app_package_maker.dart';
import 'package:flutter_app_packager/src/makers/custom/make_custom_config.dart';
import 'package:shell_executor/shell_executor.dart';

class AppPackageMakerCustom extends AppPackageMaker {
  AppPackageMakerCustom(String platform) {
    _platform = platform;
  }

  late String _platform;

  @override
  String get name => 'custom';

  @override
  String get platform => _platform;

  /// The initial packageFormat used to locate the make_config.yaml file at
  /// `<platform>/packaging/custom/make_config.yaml`.  The actual output
  /// extension is determined by [MakeCustomConfig.outputExtension] after
  /// loading the configuration file.
  @override
  String get packageFormat => 'custom';

  @override
  MakeConfigLoader get configLoader {
    return MakeCustomConfigLoader()
      ..platform = platform
      ..packageFormat = packageFormat;
  }

  @override
  Future<MakeResult> make(MakeConfig config) async {
    final customConfig = config as MakeCustomConfig;

    final String effectiveOutputPath = customConfig.packageFormat.isEmpty
        ? customConfig.outputArtifactPath
        : customConfig.outputFile.path;

    final environment = {
      ...Platform.environment,
      'APP_NAME': config.appName,
      'APP_VERSION': config.appVersion.toString(),
      'BUILD_NAME': config.appBuildName,
      if (config.appBuildNumber != null) 'BUILD_NUMBER': config.appBuildNumber!,
      'BUILD_MODE': config.buildMode,
      if (config.flavor != null) 'FLAVOR': config.flavor!,
      if (config.channel != null) 'CHANNEL': config.channel!,
      'BUILD_OUTPUT_DIRECTORY': config.buildOutputDirectory.path,
      'OUTPUT_DIRECTORY': config.outputDirectory.path,
      'OUTPUT_ARTIFACT_PATH': effectiveOutputPath,
    };

    final ProcessResult result;
    if (Platform.isWindows) {
      result = await $(
        'cmd',
        ['/c', customConfig.script],
        environment: environment,
      );
    } else {
      result = await $(
        'sh',
        ['-c', customConfig.script],
        environment: environment,
      );
    }

    if (result.exitCode != 0) {
      throw MakeError(result.stderr?.toString());
    }

    return resultResolver.resolve(config);
  }
}
