import 'package:flutter_app_packager/src/makers/rpm/app_package_maker_rpm.dart';
import 'package:test/test.dart';

void main() {
  group('sanitizeRpmRpath', () {
    test('replaces a single absolute build path', () {
      expect(sanitizeRpmRpath('/mnt/project/lib'), r'$ORIGIN');
    });

    test('removes absolute paths from mixed RPATH values', () {
      expect(
        sanitizeRpmRpath(r'$ORIGIN:/mnt/project/lib'),
        r'$ORIGIN',
      );
      expect(
        sanitizeRpmRpath(r'/mnt/project/lib:$ORIGIN'),
        r'$ORIGIN',
      );
    });

    test('preserves relative and origin-based entries', () {
      expect(
        sanitizeRpmRpath(r'$ORIGIN:$ORIGIN/../lib:plugins'),
        r'$ORIGIN:$ORIGIN/../lib:plugins',
      );
    });

    test('preserves an empty RPATH', () {
      expect(sanitizeRpmRpath(''), isEmpty);
    });
  });
}
