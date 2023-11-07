extends Control

@export var scroll_text_debug:bool = true
@onready var scroll_label = find_child("scrollLabel") as RichTextLabel
@onready var scroller = find_child("Scroller") as ScrollContainer
@onready var j1_gustos = find_child("j1boxes").get_children()
@onready var j2_gustos = find_child("j2boxes").get_children()
@onready var tvar = find_child("tvar")
@onready var nvar = find_child("nvar")
@onready var infoPerf = find_child("infoPerf") as CheckBox
@onready var j2m = find_child("j2m") as CheckBox

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
	
	scroll_label.clear()
	
	var res = sim.empezar_simulacion(gustos1, gustos2, t, n, infoPerf.button_pressed, j2m.button_pressed)
	
	# sim.testeo_prints(n)
	
	# print("Resultados: ", res)
	
	escribir_output(res)

func controlar_gustos(gustos):
	pass

func _on_sim_torta_mandar_output(output):
	scroll_label.add_text(output)
	scroller.scroll_vertical = scroller.get_v_scroll_bar().max_value

func escribir_output(data):
	
	var info_p = ""
	
	if infoPerf.button_pressed:
		info_p = "perfecta"
	
	var j2_malvado = ""
	
	if j2m.button_pressed:
		j2_malvado = "malvado"
	
	var nombre_ar:String = "data/" + nvar.text + "_" + tvar.text + "_" + str(Time.get_ticks_msec()) + "_" + info_p + "_" + j2_malvado + ".csv"
	
	var archivo_out = FileAccess.open(nombre_ar, FileAccess.WRITE)
	
	archivo_out.store_string("nro_partida,t,res_j1,res_j2,ganancia_j1,ganancia_j2\n")
	
	var nro_partida = 1
	
	for item in data:
		archivo_out.store_string(str(nro_partida) + "," + tvar.text + "," 
		+ str(item[0]) + "," + str(item[1]) + "," 
		+ str(item[2]) + "," + str(item[3]) + "\n")
		
		nro_partida += 1
	
	archivo_out.close()
