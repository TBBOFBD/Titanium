package net.titanium.utils.result;

import java.util.function.*;
import java.util.*;

public final class Result<S> implements IResult<S> {
    /**
     * Create a successful Result.
     *
     * @param s the result value, must be non-null.
     * @return the successful result.
     */
    public static <S> Result<S> success(S s) {
        return new Result<>(null, Objects.requireNonNull(s));
    }
    /**
     * Create a failed Result.
     *
     * @param e the error, must be non-null.
     * @return the erroneous Result.
     */
    public static <S> Result<S> fail(Exception e) {
        return new Result<>(Objects.requireNonNull(e), null);
    }
    /**
     * Creates a Result based on what happens when the given
     * supplier is called. If a value is returned, the Result
     * will be successful. If an exception is thrown, the
     * Result will be a failure.
     *
     * @param s the value supplier.
     * @return the Result.
     */
    public static <S> Result<S> from(ThrowingSupplier<S> s) {
        Objects.requireNonNull(s);
        try {
            return success(s.get());
        } catch (Exception t) {
            return fail(t);
        }
    }
    private final Exception e;
    private final S s;
    private Result(Exception e, S s) {
        this.e = e;
        this.s = s;
    }
    @Override
    public boolean isError() {
        return e != null;
    }
    @Override
    public boolean isSuccess() {
        return s != null;
    }
    @Override
    public Exception getError() {
        if (!isError()) throw new NoSuchElementException("Attempted to retrieve error on non-erroneous result");
        return e;
    }
    @Override
    public S get() {
        if (isError()) throw new NoSuchElementException("Attempted to retrieve value on erroneous result");
        return s;
    }
    @Override
    public S getOrElse(S def) {
        if (isError()) return def;
        return s;
    }
    @Override
    public S getOrElse(Function<Exception, S> f) {
        if (isError()) return f.apply(e);
        return s;
    }
    @Override
    public S getOrThrow() throws Exception {
        if (isError()) throw e;
        return s;
    }
    @Override
    @SuppressWarnings("unchecked")
    public <N> IResult<N> map(Function<S, N> f) {
        if (isError()) return (Result<N>)this;
        return Result.from(() -> f.apply(s));
    }
    @Override
    public IResult<S> mapError(Function<Exception, Exception> f) {
        if (isError()) return Result.fail(f.apply(e));
        return this;
    }
    @Override
    public IResult<S> ifError(Consumer<Exception> consumer) {
        if (isError()) consumer.accept(e);
        return this;
    }
    @Override
    public IResult<S> ifSuccess(Consumer<S> consumer) {
        if (isSuccess()) consumer.accept(s);
        return this;
    }
    @Override
    public IResult<S> wrapError(BiFunction<String, Exception, Exception> f, String message) {
        if (isError()) return Result.fail(f.apply(message, e));
        return this;
    }
    @Override
    public IResult<S> wrapError(Function<Exception, Exception> f) {
        return mapError(f);
    }
    @Override
    public Optional<S> asOptional() {
        return Optional.ofNullable(s);
    }

    @Override
    public IResult<S> handleErrorNow(Function<Exception, S> func) {
        if(isError()) return Result.success(func.apply(e));
        assert isSuccess();
        return this;
    }
}