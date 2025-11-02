import http.server
import socketserver
import os

from urllib.parse import urlparse, unquote

PORT = 8080

# Define MIME types for compressed files
MIME_TYPES = {
    '.wasm': 'application/wasm',
    '.js': 'application/javascript',
    '.css': 'text/css',
    '.html': 'text/html',
}

class GzipSimpleHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    """
    A custom handler that checks for and serves a pre-gzipped version of a file.
    """
    def translate_path(self, path):
        # Decode the path
        path = path.split('?',1)[0]
        path = path.split('#',1)[0]
        trailing_slash = path.endswith('/')
        path = super().translate_path(path)
        if trailing_slash:
            return os.path.join(path, 'index.html')
        return path

    def send_response_file(self, filename):
        # 1. Check if the browser accepts Gzip
        accepts_gzip = 'gzip' in self.headers.get('Accept-Encoding', '')

        # 2. Check for a pre-gzipped file existence
        if accepts_gzip and filename.endswith(tuple(MIME_TYPES.keys())):
            gzipped_filename = filename + '.gz'
            if os.path.exists(gzipped_filename):

                # Get the original file extension (e.g., '.wasm')
                ext = os.path.splitext(filename)[1]
                content_type = MIME_TYPES.get(ext, 'application/octet-stream')

                # 3. Serve the gzipped file
                with open(gzipped_filename, 'rb') as f:
                    self.send_response(200)
                    self.send_header("Content-type", content_type)
                    self.send_header("Content-Encoding", "gzip")
                    self.send_header("Content-Length", os.fstat(f.fileno()).st_size)
                    self.end_headers()
                    self.copyfile(f, self.wfile)
                return True # File served

        return False # Fall back to default behavior

    def do_GET(self):
        f = self.send_response_file(self.translate_path(self.path))
        if not f:
            # Fallback to the parent's (SimpleHTTPRequestHandler's) do_GET if no gzip file was served
            super().do_GET()


# --- Execution ---
print(f"Serving at: http://127.0.0.1:{PORT}")
print("Pre-Gzipped files will be served with Content-Encoding: gzip if they exist and client supports it.")

Handler = GzipSimpleHTTPRequestHandler
with socketserver.TCPServer(("", PORT), Handler) as httpd:
    httpd.serve_forever()
