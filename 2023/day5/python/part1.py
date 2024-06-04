
from collections import OrderedDict

path = "../test1.txt"
# path = "../prod1.txt"
# path = "../test2.txt"
# path = "../prod2.txt"



# def process_block(block):
#     # First, need to split  in the "-" character, but the first line only
#     source,_,destination, *_ = block[0].split("-")
#     return source, destination



def convert_ranges(source_category:str, source:int, destination_category:str, destination:int) -> None:
    pass

def category_result(numeric_input:int, category:str, final_mappings:dict[str,list[int]]) -> int:
    try:
        return final_mappings[category][numeric_input]
    except: 
        return numeric_input




# def source_to_destination(source, destination, range):

with open(path, "r") as file:
    # NOTE: start from zero (the max range), and overwrite the locations in which we iterate through the overlapping locations
    # NOTE: need to calculate the max between the (source_range + range, destination_range + range)

    # This is correct! Corretly have the blocks and the lines inside of 'em
    blocks =[line for line in file.read().strip().split("\n\n")] 
    blocks = [block.split("\n") if index > 0 else block for index,block in enumerate(blocks)]


    seeds, blocks = blocks[0], blocks[1:]
    # NOTE: ignore lsp warning
    seeds = [int(seed) for seed in seeds.split(":")[1].strip().split(" ")]

    # Dict wherer to add the keys and the subkeys, to go from the first string to the last
    # final_mappings = {}  # add the first row keys
    final_mappings = OrderedDict() # add the first row keys
    categories = []


    for block in blocks:
        string_mappings = block[0] # the 

        # NOTE: maybe is not necessary, since we can just take the space split with the dots as category
        # source_category, _, destination_category, *_ = string_mappings.replace("-"," ").split(" ") 
        category = string_mappings.split(" ")[0]
        categories.append(category)

        max_length = 0
        initial_array=[]
        # Need  to see where the source maximum is the maximum of the previous one
        for index,row in enumerate(block[1:]):

            destination, source, length = [int(number) for number in row.split(" ")]
            max_length= max(destination+length, source+length) # for a range function

            # Now, need to calculate the max number between the two.
            #NOTE: this is for a range (since for 99 its 100, meaning that the range wont include it)
            min_lenght = min(destination, source)


            initial_array = [i for i in range(max_length)] # its ok, shouldnt include the right side
            initial_array[source:source+length] = [i for i in range(destination, destination+length)]
            # NOTE: this is correct, but howto 



        final_mappings.update({category:initial_array})
        # for seed in seeds:
        #     print(final_mappings[0][seed])




    # # NOTE: this yield the correct results, but how to chain them together?
    # print(final_mappings["water-to-light"][81]) 
    # print(final_mappings["water-to-light"][53])
    # print(final_mappings["light-to-temperature"][74])



    # print(final_mappings["humidity-to-location"][78])
    print(final_mappings["humidity-to-location"])
    # results = []
    # for seed in seeds[1:]:
    #     for k, v in final_mappings.items():
    #         # print(f"{seed=} - {final_mappings[k][seed]=}")
    #         if seed <= len(v):
    #             seed = final_mappings[k][seed]
    #         else:
    #             print(f"{k=} - {v=}")
    #             seed = seed
    #         print(f"KEY {k} -  for seed: {seed}")
    #     results.append(seed)
    # print(results)


    # final_results = []
    # for seed in seeds:
    #     seed_final_result = category_result(numeric_input=seed, category=categories[0], final_mappings=final_mappings)
    #     for category in categories[1:]:
    #         print(category_result(numeric_input=seed_final_result,category=category, final_mappings=final_mappings))
    #     break



    # print(categories)
    # print(final_mappings)
