package xy.ai.mcpc.ast.engine;

import com.github.javaparser.ast.CompilationUnit;
import com.sun.net.httpserver.HttpServer;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.stream.Stream;

/**
 * Starts the JavaParser AST engine as a standalone JSON/HTTP backed engine.
 */
public final class Main {

  private Main() {
  }

  public static void main(String[] args) throws Exception {
    if (args.length > 0 && "convert".equals(args[0])) {
      if (args.length < 2) {
        System.err.println("usage: convert <directory>");
        System.exit(1);
        return;
      }
      convert(Paths.get(args[1]));
      return;
    }
    int port = args.length > 0 ? Integer.parseInt(args[0]) : Integer.parseInt(System.getenv().getOrDefault("PORT", "8787"));
    JavaAstServer server = new JavaAstServer();
    HttpServer http = server.start(port);
    System.out.println("java-javaparser ast engine listening on port " + http.getAddress().getPort());
  }

  /**
   * Recursively re-parses and re-prints every {@code .java} file under {@code root}, normalising its formatting.
   */
  private static void convert(Path root) throws IOException {
    JavaAstEngine engine = new JavaAstEngine();
    try (Stream<Path> paths = Files.walk(root)) {
      paths.filter(p -> p.toString().endsWith(".java") && Files.isRegularFile(p)).forEach(p -> {
        try {
          String source = Files.readString(p);
          CompilationUnit cu = engine.parseCompilationUnit(source);
          Files.writeString(p, engine.print(cu));
          System.out.println("converted " + p);
        } catch (Exception e) {
          System.err.println("failed to convert " + p + ": " + e.getMessage());
        }
      });
    }
  }
}
