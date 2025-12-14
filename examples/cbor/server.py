# server.py
from http.server import BaseHTTPRequestHandler, HTTPServer


class EchoHandler(BaseHTTPRequestHandler):
    def do_POST(self):
        length = int(self.headers.get("Content-Length", 0))
        body = self.rfile.read(length)

        self.send_response(200)
        self.send_header("Content-Type", "application/cbor")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()

        self.wfile.write(body)

    # optional: silence default logging
    def log_message(self, format, *args):
        pass


if __name__ == "__main__":
    HTTPServer(("0.0.0.0", 8080), EchoHandler).serve_forever()
