Die Tool Discovery soll verbessert werde. AKtuelle werden die Tools eines Modells im `AISessionManager` lazy bei auswahl eines Modells gesetzt. Das soll entfernt werden.
Die `ModelResolverRegistry` soll in Zukunft beim Modell Resolving die Tools aller Modelle setzen. Der MCP-Client ist nicht initial verfügbar, daher soll die Registry selbst eine Version der Liste halten.
Die Toolliste soll bei der ersten Modellauflösung geladen werden und nur nach einem Update gespeichert werden auf Basis der StateLocation.
Der MCPClient soll einen Connect Observer erhalten den die ModelResolverRegistry verwendet. Bei der ersten Verbindung des MCPClient soll der Observer benachrichtigt werden. Darauf holt die Registry die aktuelle Toolliste, aktualisiert alle Modell und speichert die Liste, jedoch nur wenn es Änderungen zur internen Liste gibt.

- Session Manager: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/AISessionManager.java`
- Activator: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Activator.java`
- Registry: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/ModelResolverRegistry.java`
- MCPClient: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPClient.java`

Beispielcode:
```java
import java.io.File;
import java.io.IOException;
import java.nio.file.Files;

import org.eclipse.core.runtime.IPath;
import org.eclipse.core.runtime.Plugin;
import org.osgi.framework.BundleContext;

public class Activator extends Plugin {

    public static final String PLUGIN_ID = "com.example.myplugin";

    private static Activator plugin;

    @Override
    public void start(BundleContext context) throws Exception {
        super.start(context);
        plugin = this;
    }

    @Override
    public void stop(BundleContext context) throws Exception {
        plugin = null;
        super.stop(context);
    }

    public static Activator getDefault() {
        return plugin;
    }

    public void saveData(String content) throws IOException {
        IPath stateLocation = getStateLocation();

        File file = stateLocation
                .append("data.txt")
                .toFile();

        Files.writeString(file.toPath(), content);
    }

    public String loadData() throws IOException {
        File file = getStateLocation()
                .append("data.txt")
                .toFile();

        if (!file.exists()) {
            return null;
        }

        return Files.readString(file.toPath());
    }
}
```