package io.github.imurx.arboard;

import org.junit.Test;
import static org.junit.Assert.*;

public class LibraryTest {
    @Test
    public void setText() {
        try (Clipboard clip = new Clipboard()) {
            clip.setText("input");
            assertEquals("Clipboard text isnt equal", "input", clip.getText());
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    private static final byte[] image = {127,127,127,127,
        127,127,127,127,
    };
    @Test
    public void setImage() {
        try (Clipboard clip = new Clipboard()) {
            ImageData data = new ImageData(2, 1, image);
            clip.setImage(data);
            Thread.sleep(5000);
            ImageData data2 = clip.getImage();
            assertArrayEquals(image, data2.getImage());
            data.close();
            data2.close();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}
