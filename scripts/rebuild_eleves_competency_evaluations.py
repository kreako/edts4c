from peewee import *

database = PostgresqlDatabase('postgres', **{'host': 'localhost', 'user': 'postgres'})

class UnknownField(object):
    def __init__(self, *_, **__): pass

class BaseModel(Model):
    class Meta:
        database = database

class DieselSchemaMigrations(BaseModel):
    run_on = DateTimeField(constraints=[SQL("DEFAULT CURRENT_TIMESTAMP")])
    version = CharField(primary_key=True)

    class Meta:
        table_name = '__diesel_schema_migrations'

class Domains(BaseModel):
    rank = IntegerField()
    title = TextField()

    class Meta:
        table_name = 'domains'

class Components(BaseModel):
    domain = ForeignKeyField(column_name='domain_id', field='id', model=Domains)
    rank = IntegerField()
    title = TextField()

    class Meta:
        table_name = 'components'

class Competencies(BaseModel):
    c1 = TextField(null=True)
    c2 = TextField(null=True)
    c3 = TextField(null=True)
    c4 = TextField(null=True)
    component = ForeignKeyField(column_name='component_id', field='id', model=Components)
    rank = IntegerField()
    title = TextField()

    class Meta:
        table_name = 'competencies'

class Eleves(BaseModel):
    active = BooleanField()
    birthdate = DateField()
    cycle = TextField()
    firstname = TextField()
    lastname = TextField()
    school_entry = DateField()

    class Meta:
        table_name = 'eleves'

class Evaluations(BaseModel):
    comment = TextField(null=True)
    competency = ForeignKeyField(column_name='competency_id', field='id', model=Competencies)
    eleve = ForeignKeyField(column_name='eleve_id', field='id', model=Eleves)
    status = TextField()

    class Meta:
        table_name = 'evaluations'
        indexes = (
            (('eleve', 'competency'), True),
        )

class GeneralComments(BaseModel):
    comment = TextField(null=True)
    eleve = ForeignKeyField(column_name='eleve_id', field='id', model=Eleves, unique=True)

    class Meta:
        table_name = 'general_comments'

class KeyStore(BaseModel):
    name = TextField(primary_key=True)
    value = TextField(null=True)

    class Meta:
        table_name = 'key_store'


evaluations = []
for eleve in Eleves.select():
    for competency in Competencies.select():
        evaluations.append({'comment': None,
            'competency': competency.id,
            'eleve': eleve.id,
            'status': 'Empty',})
Evaluations.insert_many(evaluations).execute()
