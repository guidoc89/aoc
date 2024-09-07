

# path = "../test1.txt"
# path = "../test2.txt"
# path = "../prod1.txt"
# path = "../test2.txt"
path = "../prod2.txt"

numbers = ("one", 'two', "three", "four", "five","six", "seven","eight", "nine")



# colors = ("red", "green", "blue")

id_sum = 0
part_two_sum = 0
with open(path, "r") as file:
    lines = [line.replace("\n","") for line in file.readlines()]

    for line in lines:

        game_id, info = line.split(":")
        game_id = game_id.replace("Game","").replace(" ","")
        # print(game_id)
        # info = info.replace(" ","").replace(",","").replace(";","")
        
        mini_games = info.split(";")
        mapping = {
            "blue":0,
            "red":0,
            "green":0,
        }
        for game in  mini_games:
            individual =  game.split(",")
            for single in individual:
                for color_string, value in mapping.items():
                    if color_string in str(single).lower():
                        # This is the numeric value associated with the color
                        single = single.replace(color_string,"").replace(" ","")

                        # mapping[color_string] += int(single)
                        print(f"Current color string: {color_string} - current single - {single} - current value - {value}")
                        if int(single) > value:
                            # print(f"current single {single} is less than {value}")
                            mapping[color_string] = int(single)
                            # print(f"{mapping[color_string]=}")

            # if mapping["blue"] <= 14 and mapping["green"] <=13 and mapping["red"] <= 12:
            #     success += 1

        part_two_sum += (mapping["blue"] * mapping["red"] * mapping["green"])
        # print(f'{mapping["blue"]=}')
        # print(f'{mapping["green"]=}')
        # print(f'{mapping["red"]=}')
        # print(mapping["blue"] * mapping["red"] * mapping["green"])
        # break

        # if success == len(mini_games):
        #     id_sum += int(game_id)



# print(id_sum)
print(part_two_sum)









    

