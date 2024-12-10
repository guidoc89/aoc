
# path = "../test1.txt"
# path = "../prod1.txt"
# path = "../test2.txt"
path = "../prod2.txt"

from functools import reduce 



# def calcula(x):
#     return x*2
# scoring =  lambda x:  x*2
total_score = 0


def card_number_matches(winning_numbers:list[str], my_numbers:list[str]) -> int:
    number_matches = 0
    for winning_number in winning_numbers:
        if winning_number in my_numbers:
            number_matches += 1
    return number_matches

# score = lambda number_matches: 
initial_cards = []
with open(path, "r") as file:
    cards = [card.replace("\n","") for card in file.readlines()]
    # Len lines gives us the max Y coord, while the max X will be given by any particular line len

    # Array with the count of the amount of times a particular Card game needs to be done
    initial_cards = [1]* len(cards)
    # print(f"{initial_cards=}")


    for index, card in enumerate(cards):
        should_calculate = True

        # This should be done and X amount of time, the amount of counts for this particular Card, that depends on the array
        for trie in range(initial_cards[index]):
            number_matches = 0
            winning_numbers, my_numbers = card.split("|")
            winning_numbers = winning_numbers.split(":")[1].strip()
            winning_numbers = [number.strip() for number in winning_numbers.split(" ") if len(number)>= 1]
            my_numbers = my_numbers.strip()
            my_numbers = [number.strip() for number in my_numbers.split(" ") if len(number)>= 1]
            
            # NOTE: this returns the number of matches for the current card, incrementing the +1 count of all the indexes from the current one
            number_matches  = card_number_matches(winning_numbers, my_numbers)
            # initial_cards[index:number_matches+1] += 
            initial_cards = [number_tries+1 if cards_index > index and cards_index <= (index+number_matches) else number_tries for cards_index, number_tries in enumerate(initial_cards)]



            # Check the amount of matches and score accordingly
            # print(f"Number of matches: {number_matches}")


            # # NOTE: this is for part 1
            # if number_matches == 1:
            #     total_score += 1
            # elif number_matches >= 1:
            #     total_score +=reduce(lambda x,y: x*2, [2]*(number_matches-1))

        # Need to find the first match

        # print(f" {winning_numbers=}")
        # print(f" {my_numbers=}")


    # print(initial_cards)
    print(reduce(lambda x,y: x+y, initial_cards))
    # print(lines)

# guido = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17]
# print([card+1 if position >= index and position < (index+number_matches) else card for  position, card in enumerate(guido)])
# guido[:2] += 1
# print(guido)

# print(total_score)
# print(reduce(lambda x,y: x*2, [2]*(4-1)))

# print([0]*3)

# # print(67 in [1,2,3,67])
# lista = [1,2,3,4,5]
# # print(reduce(lambda x,y: x*2, lista))
# print([1]*3)
