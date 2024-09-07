from pprint import PrettyPrinter 

pp = PrettyPrinter()
# path = "../test1.txt"
# path = "../prod1.txt"
# path = "../test2.txt"
path = "../prod2.txt"



# TODO:  maybe try to get all of the numbers based on the x coord (if they are adjacent?) -> maybe its better to try to get the numbers first?
# NOTE:  would it be better to try to get the numbers beginning and end position?  
numbers = []
symbols = []
with open(path, "r") as file:
    # Now, need to track the symbols, not the numbers...
    lines = [line.replace("\n","")+"." for line in file.readlines()] 
    for y, line in enumerate(lines):
        left = 0
        scan = 0
        current_number = ""
        while scan <= len(line) - 1:
            char = str(line[scan])
            if not char.isdigit() and char != ".":
                # NOTE: add the type of symbol, to check later
                symbols.append({"x":scan, "y":y, "symbol":char})
            if str(line[scan]).isdigit():
                current_number += str(line[scan])
                scan += 1
            else:
                if len(current_number) != 0:
                    # numbers.append({"number:"current_number, "bbox":{"xo":}})
                    # numbers.append({"number":current_number, "x0":left-1, "x1":scan, "y0":y-1, "y1":y+1})
                    # print(f"Scan: {scan=} position for number {current_number=}")
                    numbers.append({"number":current_number, "bbox":{"x0":left-1, "x1":scan, "y0":y-1, "y1":y+1}})
                current_number = ""
                scan += 1
                left = scan

def is_symbol_in_bbox(bbox:dict[str,int], symbol_coord:dict[str,int]) -> bool:
    x0, x1, y0, y1 = bbox["x0"], bbox["x1"], bbox["y0"], bbox["y1"]
    x_symbol, y_symbol = symbol_coord["x"], symbol_coord["y"]

    # print(f"{x0=}-{x1=}-{x_symbol=}-{y0=}-{y1=}-{y_symbol=}")

    # print("-"*80)
    # print(f'X data: {x_symbol=}  {x0=}  {x1=}')
    # print(f'Y data: {y_symbol=}  {y0=}  {y1=}')
    if x_symbol in range(x0, x1+1) and y_symbol in range(y0, y1+1):
        return True
    return False


#
# pp.pprint(len(numbers))
# pp.pprint(len(symbols))
# pp.pprint(numbers)
# pp.pprint(symbols)

# print((3 in (-1,3)) and (1 in (-1,1)))

# print([number for number in numbers if number["number"] === "850"]) 
# print([number["number"] for number in numbers if number["bbox"]["y0"]==125])

# for symbol in symbols:
#     # if any([is_symbol_in_bbox(bbox=number["bbox"], symbol_coord=symbol) for number in numbers]): # NOTE: this doesnt work, cant extract the number later (since i cant access the number inside of the list comprh)
#     # # #     # print(f'{number["bbox"]=}')
#     # # #     # print(f'{number["number"]=}')
#     #     total_sum += int(number["number"])
#     #     break
#
#     for number in numbers:
#         if is_symbol_in_bbox(bbox=number["bbox"], symbol_coord=symbol):
#             print(f"{number['number']=}")
#             total_sum += int(number["number"])
#     #     # print(f"{symbol=}")
#     #     # print(f"{numb]=}")
#     #     if is_symbol_in_bbox(bbox=number["bbox"], symbol_coord=symbol):
#     #         print(f'Number: {number["number"]=}---{number["bbox"]=}-{symbol=}')
#     #         total_sum += int(number["number"])
#     #         pass



# print(f"{example_number_number=}")
# print(f"{example_number_coord=}")
# print(f"{example_symbol=}")
# print(is_symbol_in_bbox(bbox=example_number_coord, symbol_coord=example_symbol))

total_sum=0
for symbol in symbols:
    if symbol["symbol"] == "*":
        possible_parts = []
        count = 0
        for number in numbers:
            # is_gear = False
            if is_symbol_in_bbox(bbox=number["bbox"], symbol_coord=symbol):
                # total_sum += int(number["number"])
                # total_sum += int(number["number"])
                possible_parts.append(int(number["number"]))
        if len(possible_parts) == 2:
            total_sum += possible_parts[0]*possible_parts[1]
print(total_sum)


# print(total_sum)
# print(numbers[0].items())
# print(numbers)
# [print(number["bbox"]) for number in numbers]
# print(symbols)



# print(f'{numbers[1]["bbox"]=}-{symbols[0]=}')
# print(is_symbol_in_bbox(bbox=numbers[1]["bbox"], symbol_coord=symbols[0]))
# print(is_symbol_in_bbox())
