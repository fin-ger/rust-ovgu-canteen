var searchIndex = {};
searchIndex["ovgu_canteen"] = {"doc":"","items":[[0,"ovgu","ovgu_canteen","This module can gather information provided by the webpages of the Otto-von-Guericke University Magdeburg (ovgu). This module currently provides access to the canteen menus and is able to serialize the information to a json string.",null,null],[4,"Error","ovgu_canteen::ovgu","The `Error` enum represents several different error types that are used by results in this crate.",null,null],[13,"Creation","","This error is used when a creation of something failed.",0,null],[13,"NotAvailable","","This error is used when something is not available or cannot be found.",0,null],[13,"InvalidValue","","This error is used when invalid data got passed.",0,null],[0,"canteen","","The `ovgu::canteen` module is able to parse the ovgu canteen website and provide the information as serializable and deserializable structs.",null,null],[3,"Canteen","ovgu_canteen::ovgu::canteen","A canteen holds all the meals on all available days.",null,null],[12,"description","","This identifies a canteen.",1,null],[12,"days","","All available days holding the meals for this canteen.",1,null],[3,"Day","","A `Day` holds all the meals that are available at the given day.",null,null],[12,"date","","The date of thsi day.",2,null],[12,"meals","","The meal available on this day.",2,null],[12,"side_dishes","","The side dishes available on this day.",2,null],[3,"Meal","","A `Meal` holds the meals name, the price, several symbols, additives, and allergenics.",null,null],[12,"name","","The name of the meal.",3,null],[12,"price","","The price of the meal.",3,null],[12,"symbols","","Symbols that the meal is annotated with.",3,null],[12,"additives","","Additives of the meal.",3,null],[12,"allergenics","","Allergenics contained in the meal.",3,null],[3,"Price","","This struct represents the price of a meal.",null,null],[12,"student","","The price for students.",4,null],[12,"staff","","The price for staff.",4,null],[12,"guest","","The price for guests.",4,null],[4,"Additive","","An `Additive` is used to represent additives of a meal.",null,null],[13,"FoodColoring","","",5,null],[13,"FoodPreservatives","","",5,null],[13,"AntiOxidants","","",5,null],[13,"FlavorEnhancer","","",5,null],[13,"Sulfurized","","",5,null],[13,"Waxed","","",5,null],[13,"Blackend","","",5,null],[13,"Phosphates","","",5,null],[13,"Sweetener","","",5,null],[13,"Phenylalanine","","",5,null],[4,"Allergenic","","This enum represents allergenics that are contained in a meal.",null,null],[13,"Wheat","","",6,null],[13,"Rye","","",6,null],[13,"Barley","","",6,null],[13,"Oat","","",6,null],[13,"Spelt","","",6,null],[13,"Kamut","","",6,null],[13,"Crustacean","","",6,null],[13,"Egg","","",6,null],[13,"Fish","","",6,null],[13,"Peanut","","",6,null],[13,"Soya","","",6,null],[13,"Lactose","","",6,null],[13,"Almond","","",6,null],[13,"Hazelnut","","",6,null],[13,"Walnut","","",6,null],[13,"Cashew","","",6,null],[13,"PecanNut","","",6,null],[13,"BrazilNut","","",6,null],[13,"Pistachio","","",6,null],[13,"MacadamiaNut","","",6,null],[13,"QueenslandNut","","",6,null],[13,"Celery","","",6,null],[13,"Mustard","","",6,null],[13,"Sesame","","",6,null],[13,"Sulphite","","",6,null],[13,"Lupin","","",6,null],[13,"Mollusc","","",6,null],[4,"CanteenDescription","","This enum identifies a canteen.",null,null],[13,"Downstairs","","The canteen located downstairs.",7,null],[13,"Upstairs","","The canteen located upstairs.",7,null],[4,"Symbol","","This enum represents symbols a meal is annotated with.",null,null],[13,"Pig","","",8,null],[13,"Cattle","","",8,null],[13,"Poultry","","",8,null],[13,"Fish","","",8,null],[13,"Game","","",8,null],[13,"Lamb","","",8,null],[13,"Vegan","","",8,null],[13,"Bio","","",8,null],[13,"Vegetarian","","",8,null],[13,"Alcohol","","",8,null],[13,"SoupOfTheDay","","",8,null],[13,"MensaVital","","",8,null],[13,"Garlic","","",8,null],[13,"AnimalWelfare","","",8,null],[11,"fmt","","",5,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",5,{"inputs":[{"name":"self"},{"name":"additive"}],"output":{"name":"bool"}}],[11,"clone","","",5,{"inputs":[{"name":"self"}],"output":{"name":"additive"}}],[11,"from_str","","",5,{"inputs":[{"name":"str"}],"output":{"name":"result"}}],[11,"fmt","","",6,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",6,{"inputs":[{"name":"self"},{"name":"allergenic"}],"output":{"name":"bool"}}],[11,"clone","","",6,{"inputs":[{"name":"self"}],"output":{"name":"allergenic"}}],[11,"from_str","","",6,{"inputs":[{"name":"str"}],"output":{"name":"result"}}],[11,"fmt","","",8,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",8,{"inputs":[{"name":"self"},{"name":"symbol"}],"output":{"name":"bool"}}],[11,"clone","","",8,{"inputs":[{"name":"self"}],"output":{"name":"symbol"}}],[11,"from_str","","",8,{"inputs":[{"name":"str"}],"output":{"name":"result"}}],[11,"fmt","","",4,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",4,{"inputs":[{"name":"self"},{"name":"price"}],"output":{"name":"bool"}}],[11,"ne","","",4,{"inputs":[{"name":"self"},{"name":"price"}],"output":{"name":"bool"}}],[11,"clone","","",4,{"inputs":[{"name":"self"}],"output":{"name":"price"}}],[11,"from_str","","",4,{"inputs":[{"name":"str"}],"output":{"name":"result"}}],[11,"update","","",4,{"inputs":[{"name":"self"},{"name":"self"}],"output":{"name":"result"}}],[11,"fmt","","",3,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",3,{"inputs":[{"name":"self"}],"output":{"name":"meal"}}],[11,"from_element","","",3,{"inputs":[{"name":"elementref"}],"output":{"name":"result"}}],[11,"update","","",3,{"inputs":[{"name":"self"},{"name":"self"}],"output":{"name":"result"}}],[11,"eq","","",3,{"inputs":[{"name":"self"},{"name":"self"}],"output":{"name":"bool"}}],[11,"fmt","","",2,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",2,{"inputs":[{"name":"self"}],"output":{"name":"day"}}],[11,"from_element","","",2,{"inputs":[{"name":"elementref"}],"output":{"name":"result"}}],[11,"update","","",2,{"inputs":[{"name":"self"},{"name":"self"}],"output":{"name":"result"}}],[11,"eq","","",2,{"inputs":[{"name":"self"},{"name":"self"}],"output":{"name":"bool"}}],[11,"fmt","","",1,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",7,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",7,{"inputs":[{"name":"self"},{"name":"canteendescription"}],"output":{"name":"bool"}}],[11,"clone","","",7,{"inputs":[{"name":"self"}],"output":{"name":"canteendescription"}}],[11,"new","","This method creates a new canteen instance from a given description.",1,{"inputs":[{"name":"canteendescription"}],"output":{"name":"result"}}],[11,"update","","This method updates the canteen from the website.",1,{"inputs":[{"name":"self"}],"output":{"name":"result"}}],[11,"eq","","",1,{"inputs":[{"name":"self"},{"name":"self"}],"output":{"name":"bool"}}],[8,"FromElement","","This trait is used to create an instance from a HTML element reference.",null,null],[16,"Err","","This is the error type used when the creation from a HTML element fails.",9,null],[10,"from_element","","Create an instance of `Self` from a HTML element reference.",9,{"inputs":[{"name":"elementref"}],"output":{"name":"result"}}],[8,"Update","","This trait is used to update the data of an instance.",null,null],[16,"Err","","This is the error type used when the update fails.",10,null],[10,"update","","Update this instance from another instance with the same type.",10,{"inputs":[{"name":"self"},{"name":"self"}],"output":{"name":"result"}}],[11,"fmt","ovgu_canteen::ovgu","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"description","","",0,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[14,"ovgu_canteen_url","ovgu_canteen","",null,null],[14,"ovgu_canteen_selector","","",null,null]],"paths":[[4,"Error"],[3,"Canteen"],[3,"Day"],[3,"Meal"],[3,"Price"],[4,"Additive"],[4,"Allergenic"],[4,"CanteenDescription"],[4,"Symbol"],[8,"FromElement"],[8,"Update"]]};
initSearch(searchIndex);
