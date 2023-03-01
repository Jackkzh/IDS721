import requests
from flask import Flask, jsonify

app = Flask(__name__)

@app.route('/')
def index():
    return "Hello, welcome to temperature checker site!"

@app.route('/hello/<name>')
def hello_name(name):
    return f"Hello, {name}!"

@app.route('/city/<city>')
def hello_city(city):
    api_key = "492d808e33b4006c459e05c300cd10da"
    url = f"http://api.openweathermap.org/data/2.5/weather?q={city}&appid={api_key}"
    resp = requests.get(url)
    weather = resp.json()['main']['temp'] - 273.15
    return f"The weather in {city} is {round(weather, 2)} degrees in Celsius!"

if __name__ == '__main__':
     app.run(debug=True, host='0.0.0.0', port=8000)