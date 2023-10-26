extends Control

@export var scroll_text_debug:bool = true
@onready var scroll_label = find_child("scrollLabel")
@onready var scroller = find_child("Scroller") as ScrollContainer
@onready var j1_gustos = find_child("j1boxes").get_children()
@onready var j2_gustos = find_child("j2boxes").get_children()
@onready var tvar = find_child("tvar")
@onready var nvar = find_child("nvar")

@onready var sim = find_child("SimTorta")

func _ready():
	scroll_label.text = ""

func _process(_delta):
	if Input.is_action_pressed("ui_accept") and scroll_text_debug:
		scroll_label.text += "pepe\n"

func _on_button_pressed():
	var gustos1:Array[float] = []
	var gustos2:Array[float] = []
	
	var t:int = int(tvar.text)
	var n:int = int(nvar.text)
	
	for g in j1_gustos:
		gustos1.append(float(g.text))
		
	for g in j2_gustos:
		gustos2.append(float(g.text))
	
	print("gustos j1: ", gustos1)
	print("gustos j2: ", gustos2)
	
	scroll_label.text = ""
	
	var res = sim.empezar_simulacion(gustos1, gustos2, t, n)
	
	print("Resultados: ", res)

func controlar_gustos(gustos):
	pass

func _on_sim_torta_mandar_output(output):
	scroll_label.text += output
	scroller.scroll_vertical = scroller.get_v_scroll_bar().max_value
