import adsk.core, adsk.fusion, traceback, math

snowflake_thickness = 0.0625 # 1/16 inch, probably take from input

bottomCenter = adsk.core.point3D.create(0, 0, 0)

lines = [[[0, 0], [1, 1]], [[1, 1], [1, 2]]] # this is what the output of generateSnowflake() will look like
# maybe change to a better data structure, idk, nathan you decide. might get long

app = adsk.core.Application.get()
ui = app.userInterface

def generateSnowflake():
    # code to generate snowflake geometry, probably to randomly generate one snowflake at a time
    # call for each new snowflake to create a new set of randomized geometry
    # make different functions for different styles of generation?
    fu = "bar"
    return fu
    # lines = array of lines consisting of a start and end point for each line, in cartesian coordinates
    # sketches are defined in one plane (unless 3D sketch) so only 2 coordinates are needed for a point
    #return lines

def createNewComponent():
    # Get the active design.
    product = app.activeProduct
    design = adsk.fusion.Design.cast(product)
    rootComp = design.rootComponent
    allOccs = rootComp.occurrences
    newOcc = allOccs.addNewComponent(adsk.core.Matrix3D.create())
    return newOcc.component

def snowflakes():
    product = app.activeProduct
    design = adsk.fusion.Design.cast(product)
    
    if not design:
        ui.messageBox('the DESIGN workspace must be active when running this script')
        return
    
    current_design_type = design.designType

    global new_component
    new_component = createNewComponent()

    if new_component is None:
        ui.messageBox('New component failed to create', 'new component failed')
        return
    
    # add sketch
    sketches = new_component.sketches
    sketch = sketches.add(new_component.xYConstructionPlane)

    sketchLines = sketch.sketchCurves.sketchLines
    # why isn't .sketchlines showing up?
    # https://help.autodesk.com/view/fusion360/ENU/?guid=GUID-31A696E5-9BF1-478D-95A9-ED996AC0B93C

    for line in lines:
        point_A = line[0]
        point_B = line[1]

        centerline = sketchLines.addByTwoPoints(point_A, point_B)
        # something like this
        
        # draw all lines for one arm of snowflake

    surfaceExtrusionFeatures = new_component.features.extrudeFeatures
    # https://help.autodesk.com/view/fusion360/ENU/?guid=GUID-4010527f-7198-47ff-9c4d-509de5c595c1
    surface_extrusion = surfaceExtrusionFeatures.addSimple(sketch.profiles[0], snowflake_thickness, adsk.fusion.FeatureOperations.NewBodyFeatureOperation)
    # this might be how it works lol

    surfaceThickenFeatures = new_component.features.thickenFeatures
    # https://help.autodesk.com/view/fusion360/ENU/?guid=GUID-9d6e6272-a579-4367-a69c-48cff407a50a

    circular_pattern = new_component.features.circularPatternFeatures
    # https://help.autodesk.com/view/fusion360/ENU/?guid=GUID-c8181a3f-7c28-4f51-8f72-821548bda1ef



def run(context):
    try:
        snowflakes()

    except:
        if ui:
            ui.messageBox('Failed\n{}'.format(traceback.format_exc()))
