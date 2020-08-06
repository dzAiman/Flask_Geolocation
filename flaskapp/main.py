from flask import Flask, render_template, request, url_for
from service import addForm
import pandas as pd
import re

app = Flask(__name__)
app.config['SECRET_KEY'] = 'key'

address = ''
df = pd.read_csv('ShopAddresses.csv')
df['Postcode'] = df['ShopAddress'].str.findall(r'\d{5}').str[0]

@app.route('/')
def index():
    form = addForm()
    return render_template('index.html', form = form)


@app.route('/address', methods = ['GET','POST'])
def process_data():
    address = request.form.get('userAddress')
    df1 = df.loc[df['Postcode'] == to_postcode(address)]
    #return render_template('form.html', address = address, tables=[df1.to_html(classes='data')], header = "true")
    return render_template('form.html', address = address, column_names=df1.columns.values, row_data=list(df1.values.tolist()), zip = zip, df1 = df1)
     

def to_postcode(address):
    p = re.compile('\d{5}')
    postcode = str(p.findall(address)).strip("[]'")
    print(type(postcode))
    return postcode

if __name__ == '__main__':
    app.run()