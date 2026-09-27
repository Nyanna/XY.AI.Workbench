package xy.ai.mcpc.ast.engine;

import com.github.javaparser.ast.CompilationUnit;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.attribute.BasicFileAttributes;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.locks.ReentrantLock;

/**
 * Timestamp/content-hash validated cache of parsed {@link CompilationUnit}s, keyed by
 * absolute path — mirrors the Python {@code AstCache}: a cheap mtime+size check first,
 * a SHA-256 content check only on mismatch, and a real re-parse only if the file's
 * content actually changed outside this process. Every mutating operation writes the
 * pretty-printed source back to disk and refreshes the cache from that exact text, so
 * reported line numbers always match what's on disk.
 */
public final class DocumentCache {

    /** {@code cu} plus the exact text it was parsed from. */
    public record Entry(CompilationUnit cu, String source) {
    }

    private record CacheEntry(long mtimeMillis, long size, String contentHash, CompilationUnit cu, String source) {
    }

    private final Map<String, CacheEntry> entries = new HashMap<>();
    private final ReentrantLock lock = new ReentrantLock();
    private final JavaAstEngine engine;

    public DocumentCache(JavaAstEngine engine) {
        this.engine = engine;
    }

    public Entry get(Path path) throws IOException {
        String key = path.toString();
        lock.lock();
        try {
            CacheEntry entry = entries.get(key);
            BasicFileAttributes attrs = Files.readAttributes(path, BasicFileAttributes.class);
            long mtimeMillis = attrs.lastModifiedTime().toMillis();
            long size = attrs.size();
            if (entry != null && entry.mtimeMillis() == mtimeMillis && entry.size() == size) {
                return new Entry(entry.cu(), entry.source());
            }
            String source = Files.readString(path, StandardCharsets.UTF_8);
            String digest = sha256(source);
            if (entry != null && entry.contentHash().equals(digest)) {
                entries.put(key, new CacheEntry(mtimeMillis, size, digest, entry.cu(), entry.source()));
                return new Entry(entry.cu(), entry.source());
            }
            CompilationUnit cu = engine.parseCompilationUnit(source);
            entries.put(key, new CacheEntry(mtimeMillis, size, digest, cu, source));
            return new Entry(cu, source);
        } finally {
            lock.unlock();
        }
    }

    /** Serialises {@code cu}, writes it to {@code path} and refreshes the cache entry from that text. */
    public String save(Path path, CompilationUnit cu) throws IOException {
        String source = engine.print(cu);
        Files.writeString(path, source, StandardCharsets.UTF_8);
        CompilationUnit normalized = engine.parseCompilationUnit(source);
        BasicFileAttributes attrs = Files.readAttributes(path, BasicFileAttributes.class);
        String digest = sha256(source);
        lock.lock();
        try {
            entries.put(path.toString(), new CacheEntry(
                    attrs.lastModifiedTime().toMillis(), attrs.size(), digest, normalized, source));
        } finally {
            lock.unlock();
        }
        return source;
    }

    public void invalidate(Path path) {
        lock.lock();
        try {
            entries.remove(path.toString());
        } finally {
            lock.unlock();
        }
    }

    private static String sha256(String text) {
        try {
            MessageDigest digest = MessageDigest.getInstance("SHA-256");
            byte[] hash = digest.digest(text.getBytes(StandardCharsets.UTF_8));
            StringBuilder sb = new StringBuilder(hash.length * 2);
            for (byte b : hash) {
                sb.append(String.format("%02x", b));
            }
            return sb.toString();
        } catch (NoSuchAlgorithmException e) {
            throw new IllegalStateException(e);
        }
    }
}
