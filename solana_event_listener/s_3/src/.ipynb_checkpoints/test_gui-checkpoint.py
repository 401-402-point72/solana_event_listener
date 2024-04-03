from nicegui import ui
from random import random
import sys
import pandas as pd
import boto3
import json
from botocore.exceptions import ClientError
import logging

logger = logging.getLogger(__name__)

def list_my_buckets(s3_resource):
    print("Buckets:\n\t", *[b.name for b in s3_resource.buckets.all()], sep="\n\t")

# snippet-start:[python.example_code.s3.helper.ObjectWrapper]
class ObjectWrapper:
    """Encapsulates S3 object actions."""

    def __init__(self, s3_object):
        """
        :param s3_object: A Boto3 Object resource. This is a high-level resource in Boto3
                          that wraps object actions in a class-like structure.
        """
        self.object = s3_object
        self.key = self.object.key

    # snippet-start:[python.example_code.s3.GetObject]
    def get(self):
        """
        Gets the object.

        :return: The object data in bytes.
        """
        try:
            body = self.object.get()["Body"].read()
            logger.info(
                "Got object '%s' from bucket '%s'.",
                self.object.key,
                self.object.bucket_name,
            )
        except ClientError:
            logger.exception(
                "Couldn't get object '%s' from bucket '%s'.",
                self.object.key,
                self.object.bucket_name,
            )
            raise
        else:
            return body
    # snippet-end:[python.example_code.s3.GetObject]
        
    # snippet-start:[python.example_code.s3.ListObjects]
    @staticmethod
    def list(bucket, prefix=None):
        """
        Lists the objects in a bucket, optionally filtered by a prefix.

        :param bucket: The bucket to query. This is a Boto3 Bucket resource.
        :param prefix: When specified, only objects that start with this prefix are listed.
        :return: The list of objects.
        """
        try:
            if not prefix:
                objects = list(bucket.objects.all())
            else:
                objects = list(bucket.objects.filter(Prefix=prefix))
            logger.info(
                "Got objects %s from bucket '%s'", [o.key for o in objects], bucket.name
            )
        except ClientError:
            logger.exception("Couldn't get objects for bucket '%s'.", bucket.name)
            raise
        else:
            return objects
    # snippet-end:[python.example_code.s3.ListObjects]

def usage():
    # print("-" * 88)
    # print("Welcome to the Point72 Blockchain GUI demo!")
    # print("-" * 88)

    logging.basicConfig(level=logging.INFO, format="%(levelname)s: %(message)s")

    bucket_string = "rustbucketethereum"
    s3_resource = boto3.resource("s3")
    bucket = s3_resource.Bucket(bucket_string)

    listed_lines = ObjectWrapper.list(bucket)
    print(f"Their keys are: {', '.join(l.key for l in listed_lines)}")

    # print(listed_lines[1])
    line =  listed_lines[1]
    line_body = line.get()

    # print(f"Got object with key {line.key} and body(Metadata) {line_body}.")
    # print(line_body['Body'])

    # json_data = json.loads(line_body['Body'].decode('utf-8'))
    json_data = json.loads(line_body['Body'].read().decode('utf-8'))
    # print(json_data)
    df = pd.DataFrame(json_data)
    print(df)


if __name__ == "__main__":
    usage()

# grid = ui.aggrid({
#     'defaultColDef': {'flex': 1},
#     'columnDefs': [
#         {'headerName': 'Name', 'field': 'name'},
#         {'headerName': 'Age', 'field': 'age'},
#         {'headerName': 'Parent', 'field': 'parent', 'hide': True},
#     ],
#     'rowData': [
#         {'name': 'Alice', 'age': 18, 'parent': 'David'},
#         {'name': 'Bob', 'age': 21, 'parent': 'Eve'},
#         {'name': 'Carol', 'age': 42, 'parent': 'Frank'},
#     ],
#     'rowSelection': 'multiple',
# }).classes('max-h-40')

# def update():
#     grid.options['rowData'][0]['age'] += 1
#     grid.update()

# ui.button('Update', on_click=update)
# ui.button('Select all', on_click=lambda: grid.run_grid_method('selectAll'))
# ui.button('Show parent', on_click=lambda: grid.run_column_method('setColumnVisible', 'parent', True))

# chart = ui.highchart({
#     'title': False,
#     'chart': {'type': 'bar'},
#     'xAxis': {'categories': ['A', 'B']},
#     'series': [
#         {'name': 'Alpha', 'data': [0.1, 0.2]},
#         {'name': 'Beta', 'data': [0.3, 0.4]},
#     ],
# }).classes('w-full h-64')

# def update():
#     chart.options['series'][0]['data'][0] = random()
#     chart.update()

# ui.button('Update', on_click=update)

# ui.run()
