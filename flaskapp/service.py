from flask_wtf import FlaskForm
from wtforms import StringField, SubmitField
from wtforms.validators import DataRequired

class addForm(FlaskForm):
    userAddress = StringField('Address', validators=[DataRequired()])
    submit = SubmitField('Check')