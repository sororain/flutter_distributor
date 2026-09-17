import 'dart:io';

import 'package:flutter_app_packager/src/makers/exe/inno_setup/inno_setup_script.dart';
import 'package:path/path.dart' as p;
import 'package:shell_executor/shell_executor.dart';

const String _innoSetupEnvVar = 'INNO_SETUP_PATH';
const String _defaultInnoSetupPath = r'C:\Program Files (x86)\Inno Setup 6';

class InnoSetupCompiler {
  /// Extra environment variables passed from the caller (e.g. from distribute_options.yaml).
  /// These take priority over Platform.environment when resolving the Inno Setup path.
  static final Map<String, String> _extraEnv = {};

  /// Set extra environment variables to be read by [_resolveIsccPathImpl] with priority.
  static void setExtraEnv(Map<String, String> env) {
    _extraEnv
      ..clear()
      ..addAll(env);
  }

  /// Resolves ISCC.exe path (public static method, usable from other classes).
  ///
  /// Priority:
  /// 1. `INNO_SETUP_PATH` environment variable (from _extraEnv or Platform.environment)
  /// 2. Default path `C:\Program Files (x86)\Inno Setup 6`
  /// 3. `iscc` found in `PATH`
  static String resolveIsccPath() {
    return _resolveIsccPathImpl();
  }

  /// Resolves the path to `ISCC.exe` using the following order:
  /// 1. `INNO_SETUP_PATH` environment variable
  /// 2. Hardcoded default path (`C:\Program Files (x86)\Inno Setup 6`)
  /// 3. `iscc` command found in `PATH`
  String _resolveIsccPath() {
    return _resolveIsccPathImpl();
  }

  static String _resolveIsccPathImpl() {
    // 1. Check environment variable (extra env takes priority)
    String? envPath = _extraEnv[_innoSetupEnvVar];
    envPath ??= Platform.environment[_innoSetupEnvVar];
    if (envPath != null && envPath.isNotEmpty) {
      Directory dir = Directory(envPath);
      if (dir.existsSync()) {
        String isccPath = p.join(dir.path, 'ISCC.exe');
        if (File(isccPath).existsSync()) {
          return isccPath;
        }
      }
    }

    // 2. Check hardcoded default path
    Directory defaultDir = Directory(_defaultInnoSetupPath);
    String defaultIsccPath = p.join(defaultDir.path, 'ISCC.exe');
    if (File(defaultIsccPath).existsSync()) {
      return defaultIsccPath;
    }

    // 3. Fallback to `iscc` in PATH
    return 'iscc';
  }

  Future<bool> compile(InnoSetupScript script) async {
    String isccExecutable = _resolveIsccPath();

    // When falling back to PATH, verify `iscc` is actually available
    if (isccExecutable == 'iscc') {
      try {
        ProcessResult check = await Process.run(
          'iscc',
          ['/?'],
          runInShell: true,
        );
        if (check.exitCode != 0) {
          throw Exception(
            '`Inno Setup 6` was not installed. '
            'Please install it (https://jrsoftware.org/isinfo.php), '
            'or set the `$_innoSetupEnvVar` environment variable to the installation path.',
          );
        }
      } catch (e) {
        if (e is Exception) rethrow;
        throw Exception(
          '`Inno Setup 6` was not installed. '
          'Please install it (https://jrsoftware.org/isinfo.php), '
          'or set the `$_innoSetupEnvVar` environment variable to the installation path.',
        );
      }
    }

    File file = await script.createFile();

    ProcessResult processResult = await $(
      isccExecutable,
      [file.path],
    );

    if (processResult.exitCode != 0) {
      return false;
    }

    file.deleteSync(recursive: true);
    return true;
  }
}
