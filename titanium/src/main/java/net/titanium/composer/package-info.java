/**
 * <h1>Composer</h1>
 * <h3>Super Fast And Easy Event System</h3>
 * Example: <pre>{@code
 * public class Main {
 *     private static final String PACKAGE = "example.com";
 *     public static void main(String[] args) {
 *         System.out.println("-- STATIC --");
 *         // Creates a new EventBus
 *         IEventBus bus = EventBusFactory.getDefault(PACKAGE);
 *
 *         // Subscribes only static methods
 *         bus.subscribe(Main.class);
 *
 *         // posts event
 *         bus.post(9);
 *         bus.post("String");
 *
 *         // Unsubscribes only static methods
 *         bus.unsubscribe(Main.class);
 *
 *         System.out.println();
 *         System.out.println("-- INSTANCE --");
 *         new Main();
 *     }
 *
 *     public Main() {
 *         // Creates a new EventBus
 *         IEventBus bus = EventBusFactory.getDefault(PACKAGE);
 *
 *         // Subscribes both static and normal methods
 *         bus.subscribe(this);
 *
 *         // Posts 3 events, boolean event won't do anything because there isn't any listener that accepts boolean
 *         bus.post("Hello");
 *         bus.post(159);
 *         bus.post(true);
 *
 *         // Unsubscribes both static and normal methods
 *         bus.unsubscribe(this);
 *
 *         // Nothing will happen because all listeners were unsubscribed
 *         bus.post("Pog");
 *     }
 *
 *     @EventHandler
 *     private void onString1(String idk) {
 *         // This listener will run later than onString2 because it has smaller priority
 *         // Default priority is EventPriority.MEDIUM
 *         System.out.println("String 1: " + idk);
 *     }
 *
 *     @EventHandler(priority = EventPriority.HIGH)
 *     private static void onString2(String idk) {
 *         // This listener will run sooner than onString1 because it has bigger priority
 *         System.out.println("String 2: " + idk);
 *     }
 *
 *     @EventHandler
 *     private void onNumber(Integer a) {
 *         // Will be called
 *         System.out.println("Numba: " + a);
 *     }
 *
 *     @EventHandler
 *     private void onNumber(int a) {
 *         // Won't be called because primitives cannot be used as events
 *         System.out.println("Numba: " + a);
 *     }
 * }
 * }</pre>
 */
package net.titanium.composer;