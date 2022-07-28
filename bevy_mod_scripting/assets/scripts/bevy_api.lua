
local comp

function on_update()
    if comp == nil then

        my_component_type = world:get_type_by_name("MyComponent")
        my_resource_type = world:get_type_by_name("MyResource")
        comp = world:get_component(entity,my_component_type)
        res = world:get_resource(my_resource_type)

        print(string.format("%s",comp))
        print(string.format("%s",res))
        print(string.format("%s",res:custom_resource_method(42)))
        print(string.format("%s",comp.vec2))

        comp.u8 = 2

        if comp.option == nil then
            print(string.format("option was %s", comp.option))
            comp.option = Vec3.new(2,1,3)
            print(string.format("option[1] is now %s", comp.option[1]))
            comp.option[1] = 5
            print(string.format("and now option[1] is %s", comp.option[1]))
        end

        comp.vec_of_option_bools = {true,false,true}
        comp.vec_of_option_bools[0] = false
        comp.vec_of_option_bools:insert(1,nil)
        comp.vec_of_option_bools:push(false)

        comp.option_vec_of_bools = {false,true,false}
        comp.option_vec_of_bools[2] = true
        comp.option_vec_of_bools:insert(1,false)
        comp.option_vec_of_bools:push(true)

        comp.vec2 = comp.vec2 + comp.vec2

        comp.uvec2 = comp.uvec2 + comp.uvec2
        comp.usize = comp.vec2:min_element()
        comp.f32 = comp.f32 + comp.f32 + comp.vec2:min_element()
        comp.vec2 = Vec2.new(2,1)
        comp.vec3 = Vec3.new(0,1,0):any_orthonormal_vector() + comp.mat3.x_axis + comp.option
        comp.vec4 = Vec4.splat(3)
        comp.quat = Quat.from_xyzw(3,2,1,4)
        comp.dquat = comp.dquat * 2
        comp.my_reflect_thing.hello = "bye world!"
        a = Mat3.from_cols(Vec3.new(1,0,0),Vec3.new(0,1,0),Vec3.new(0,0,-1))

        comp.mat3[1][1] = 42
        comp.mat3.x_axis = Vec3.new(69,69,69)

        comp = world:get_component(entity,my_component_type)
        res = world:get_resource(my_resource_type)

        print(string.format("%s", comp))
        print(string.format("%s", res))

        print(#comp.vec_of_option_bools)
        print(comp.vec_of_option_bools:pop())
        print(comp.option_vec_of_bools:pop())
        for k,v in pairs(comp.vec_of_option_bools) do
            print(string.format("%s:%s",k,v))
        end
        print(#comp.option_vec_of_bools)
        for k,v in pairs(comp.option_vec_of_bools) do
            print(string.format("%s:%s",k,v))
        end

        comp.vec_of_option_bools:clear()
        print(#comp.vec_of_option_bools)

        -- print(comp.option_vec_of_bools:remove(1))
        -- print(#comp.option_vec_of_bools)

    end
end