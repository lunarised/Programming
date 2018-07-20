import javax.swing.*;
import javax.imageio.*;
import java.awt.*;
import java.io.*;
public class Engine{
	static Image iconImage;
	public static void main(String[] args){
	System.out.println("Howdy");
	try{
	iconImage = ImageIO.read(new File("Resources/Icon.jpg"));
	}catch(IOException e){
	}
	JFrame frame = new JFrame();

	frame.setIconImage(iconImage);
	frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
	frame.pack();
       frame.setVisible(true);	
	}




}
