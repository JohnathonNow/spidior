
package com.johnwesthoff.bending.spells.lightning;

import static com.johnwesthoff.bending.util.network.ResourceLoader.loadIcon;

import java.nio.ByteBuffer;
import java.util.logging.Level;
import java.util.logging.Logger;

import com.johnwesthoff.bending.Session;
import com.johnwesthoff.bending.Constants;
import com.johnwesthoff.bending.logic.World;
import com.johnwesthoff.bending.spells.Spell;

public class LightningOvercharge extends Lightning {
    int charge = 0;
    public LightningOvercharge() {
        charge = 0;
    }

    double number;
    @Override
    public void onSpawn(Session me) {
        number = 1;
        me.x = 0;
    }
}

