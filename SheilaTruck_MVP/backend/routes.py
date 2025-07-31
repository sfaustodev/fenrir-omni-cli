
from flask import jsonify, request

def setup_routes(app):
    @app.route('/fretes', methods=['GET'])
    def get_fretes():
        return jsonify([{"id": 1, "origem": "SP", "destino": "RJ", "valor": 1200}])

    @app.route('/ofertar', methods=['POST'])
    def ofertar_frete():
        data = request.json
        return jsonify({"status": "ok", "data": data}), 201
