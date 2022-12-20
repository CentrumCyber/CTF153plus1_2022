from flask import Flask, render_template_string, redirect, request
import os

app = Flask(__name__)


@app.route('/')
def home():
    return redirect("/greeting?name=anonymous")  

@app.route("/greeting")
def index():
    args = request.args
    name = args.get("name")
    template = '''
        <center><h2>Hello, {} </h2></center>
    '''.format(name)

    return render_template_string(template)

if __name__ == "__main__":
    port = int(os.environ.get('PORT', 5000))
    app.run(debug=True, host='0.0.0.0', port=port)
