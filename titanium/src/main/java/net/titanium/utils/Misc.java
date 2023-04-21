package net.titanium.utils;

import net.titanium.utils.result.Result;

public class Misc {
    public static <T> T $$(java.util.function.Supplier<T> func) {
        return func.get();
    }
    public static <T> T LoadClass(String name, Object... args){
        return Misc.<T>loadClassSafe(name).get();
    }

    @SuppressWarnings("unchecked")
    public static <T> Result<T> loadClassSafe(String name, Object... args) {
        try {
            Class<?> loader = Misc.class.getClassLoader().loadClass(name);
            return Result.success(
                (T) loader.getDeclaredConstructor(args.getClass()).newInstance(args)
            );
        } catch (Exception e) {
            return Result.fail(e);
        }
    }
}
