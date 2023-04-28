package net.titanium.utils.result;

import java.util.function.*;
import java.util.Optional;

@SuppressWarnings({"GrazieInspection","unused","JavadocBlankLines"})
public interface IResult<S> extends Supplier<S> {
    /**
     * Returns {@code true} if the Result is erroneous, {@code false}
     * otherwise.
     */
    boolean isError();
    /**
     * Returns {@code true} if the Result is successful, {@code false}
     * otherwise.
     */
    boolean isSuccess();
    /**
     * Returns the erroneous result, which is always some type of
     * {@code Exception}. If the Result is not erroneous, a
     * {@code NoSuchElementException} is thrown.
     *
     * It is expected that you always check if the result is
     * erroneous before trying to get the error value.
     *
     * @see IResult#isError()
     */
    Exception getError();
    /**
     * Returns the result value. If the Result is erroneous, a
     * {@code NoSuchElementException} is thrown.
     *
     * It is expected that you always check if the result is
     * erroneous before trying to get the value.
     *
     * @see IResult#isError()
     */
    @Override
    S get();
    /**
     * Returns the result value unless the Result is erroneous,
     * in which case a supplied default is returned.
     *
     * @param def the default to return if this Result is erroneous.
     */
    S getOrElse(S def);
    /**
     * Returns the result value unless the Result is erroneous,
     * in which case a function is called and the result of that
     * function is used.
     *
     * @param f a function that takes the error
     *    value and returns a suitable default.
     */
    S getOrElse(Function<Exception, S> f);

    /**
     * Returns the result value unless the Result is erroneous,
     * in which case the error value is thrown.
     */
    S getOrThrow() throws Exception;

    /**
     * Maps the result value to some other value.
     *
     * If the Result is erroneous, this method returns the original
     * Result without calling the mapping function. The returned
     * value is casted, but this cast is safe because the value
     * being cast is guaranteed to be {@code null}.
     *
     * @param f mapping function.
     */
    <N> IResult<N> map(Function<S, N> f);

    /**
     * Maps the error value to some other error value, which
     * must be of type {@code Exception}.
     *
     * If the Result is successful, this method returns the
     * original Result without doing anything.
     *
     * @param f mapping function.
     */
    IResult<S> mapError(Function<Exception, Exception> f);
    /**
     * If the Result is erroneous, calls the given {@code Consumer} on
     * the error result.
     *
     * @return this
     */
    IResult<S> ifError(Consumer<Exception> consumer);
    /**
     * If the Result is successful, calls the given {@code Consumer} on
     * the result.
     *
     * @return this
     */
    IResult<S> ifSuccess(Consumer<S> consumer);
    /**
     * A convenience method for wrapping an erroneous result object
     * with more information.
     *
     * <pre>
     *   Result.fail(new IOException()).wrapError(IllegalArgumentException::new, "invalid param");
     * </pre>
     *
     * If the Result is successful, this method returns the original
     * Result without doing anything.
     */
    IResult<S> wrapError(BiFunction<String, Exception, Exception> f, String message);
    /**
     * Alias for {@code Result#mapError(Function)}.
     *
     * @see IResult#mapError(Function)
     */
    IResult<S> wrapError(Function<Exception, Exception> f);
    /**
     * Converts this Result<S> to an Optional<S>, discarding
     * the error value in the process. If the Result is
     * erroneous, {@code Optional.empty()} is returned.
     */
    Optional<S> asOptional();

    /**
     * Throws the error if it exists,
     * otherwise returns a new result that will always be ok
     * @return a new always-ok result
     */
    IResult<S> handleErrorNow(Function<Exception, S> func);
}