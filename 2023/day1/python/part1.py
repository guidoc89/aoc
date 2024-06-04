

# path = "../test1.txt"
# path = "../test2.txt"
path = "../prod2.txt"


running_sum = 0


mapping = {
    "one":1,
    "two":2,
    "three":3,
    "four":4,
    "five":5,
    "six":6,
    "seven":7,
    "eight":8,
    "nine":9,
}

test_string = "two1nine"
test_string = "eightwothree"

# with open(path, "r") as file:
#     # lines = [mapping.get(str(line.replace("\n", ""))) for line in file.readlines()]
#     lines = [str(line.replace("\n","")) for line in file.readlines()]
#     print(lines)
    

        



numbers = ("one", 'two', "three", "four", "five","six", "seven","eight", "nine")

# # NOTE: Part 1
# with open(path, "r") as file:
#     lines = [line.replace("\n","") for line in file.readlines()]
#     for line in lines:
#         digits = []
#         for index,letter  in enumerate(line):
#             if letter.isdigit():
#                 digits.append(str(letter))
#             for index2, string_number in enumerate(numbers):
#                 if line[index:].startswith(string_number):
#                     digits.append(str(index2+1))
#
#         running_sum += int(digits[0] + digits[-1])
# print(running_sum)



# NOTE: Part 1
with open(path, "r") as file:
    lines = [line.replace("\n","") for line in file.readlines()]
    for line in lines:
        digits = []
        for index,letter  in enumerate(line):
            if letter.isdigit():
                digits.append(str(letter))
            for index2, string_number in enumerate(numbers):
                if line[index:].startswith(string_number):
                    digits.append(str(index2+1))

        running_sum += int(digits[0] + digits[-1])

print(running_sum)





    # for line in lines: 
        # ints_list = []
        # for char in line:


        #     try:
        #         number = int(char)
        #         ints_list.append(number)
        #     except:
        #         pass
        #
        # if len(ints_list) == 0:
        #     pass
        #
        # if len(ints_list) == 1:
        #     running_sum += int(str(ints_list[0]) + str(ints_list[0]))
        #
        # else:
        #     running_sum += int(str(ints_list[0]) + str(ints_list[-1]))



# print(running_sum)






# def input_text(input:str) -> int:
#     csv = 

