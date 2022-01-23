package io.github.imurx.arboard;

import org.junit.Test;
import static org.junit.Assert.*;

public class LibraryTest {
    @Test
    public void setText() {
        try (Clipboard clip = new Clipboard()) {
            clip.setText("input");
            Thread.sleep(5000);
            assertEquals("Clipboard text isnt equal", "input", clip.getText());
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}
