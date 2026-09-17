# App Store

The appstore target publishes your package artifacts to the [App Store Connect](https://appstoreconnect.apple.com/apps).

## Set up environment variables

requires some environment variables set up to run correctly,Choose one of the following ways.

- Username and password

Get an app-specific password：[https://support.apple.com/HT204397](https://support.apple.com/HT204397)

```
export APPSTORE_USERNAME="Login Username"
export APPSTORE_PASSWORD="App-specific password"
```

- API key

App Store Connect API: [https://developer.apple.com/documentation/appstoreconnectapi](https://developer.apple.com/documentation/appstoreconnectapi)

```
export APPSTORE_APIKEY="API key"
export APPSTORE_APIISSUER="API issuer"
```

## Usage

Run:

```
fastforge publish \
  --path dist/1.0.0+1/hello_world-1.0.0+1-ios.ipa \
  --targets appstore /
```

### Configure `distribute_options.yaml`

```yaml
variables:
  APPSTORE_USERNAME: 'xxx'
  APPSTORE_PASSWORD: 'xxx'
  # or
  # APPSTORE_APIKEY: "xxx"
  # APPSTORE_APIISSUER: "xxx"
output: dist/
releases:
  - name: dev
    jobs:
      - name: release-dev-ios
        package:
          platform: ios
          target: ipa
          build_args:
            target: lib/main.dart
            export-options-plist: ios/ExportOptions.plist
        # Publish to appstore
        publish:
          target: appstore
```

> **Note**: Make sure your `export-options-plist` has the `method` set to `app-store`. Other export methods like `ad-hoc`, `development`, or `enterprise` will cause the upload to App Store Connect to fail.

Here's an example of an `ExportOptions.plist` file with the correct settings for App Store submission:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>method</key>
    <string>app-store</string>
    <!-- ... other settings ... -->
</dict>
</plist>
```

Run:

```
fastforge release --name dev
```

## Related Links

- [Use an app-specific password](https://support.apple.com/HT204397)
- [App Store Connect api](https://developer.apple.com/documentation/appstoreconnectapi)
- [altool guide](https://help.apple.com/asc/appsaltool/)
