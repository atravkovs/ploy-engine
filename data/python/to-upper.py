
def execute(input):
    output = {
        'name': input['name'].upper(),
        'execute': len(input['name']) > 5
    }

    return output
