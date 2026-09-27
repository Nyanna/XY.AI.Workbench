package xy.ai.mcpc.ast.engine;

import com.sun.net.httpserver.HttpServer;

/** Starts the JavaParser AST engine as a standalone JSON/HTTP backed engine. */
public final class Main {

    private Main() {
    }

    public static void main(String[] args) throws Exception {
        int port = args.length > 0 ? Integer.parseInt(args[0]) : Integer.parseInt(System.getenv().getOrDefault("PORT", "8787"));
        JavaAstServer server = new JavaAstServer();
        HttpServer http = server.start(port);
        System.out.println("java-javaparser ast engine listening on port " + http.getAddress().getPort());
    }
}
